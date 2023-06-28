#!/bin/sh

# gen-rand-testcase-after-check.sh
#
# generate random testcases automatically (between resolution 5-16), with validation (compare with external command that user set)
#
# - test eth-blockies-rs binary
# - compare its output blockies data with output of external command
# - if two output matches, add results to output-file as 'confirmed expected output for seed'
#   if not match, wait for user input to check manually. press enter to add, and terminate script to cancel
#
# usage:
#   gen-rand-testcase-after-check.sh <external-cmd-test> [output-dir] [interval] [seed-type (none/eth)]
#      - <external-cmd-test>: command to execute as test. if eth-blockies output is same as external command output, test is passed and test-io result will be saved automatically
#                           '{seed}' and '{size}' is placeholder inside the command, which will be replaced with random-generated seed and blockies-size later
#
#      - [output-dir]: output directory to write test-io files. if empty, no file io occurs [default]
#      - [interval]: interval (sec) between tests [default: 0]
#      - [seed-type]: whether to generate general blockies (none) or ethereum blockies (eth) [default: none]
#
#
#   e.g 1) normal case (test, and if passed, save result test-io to [output-dir])
#    $ gen-rand-testcase-after-check.sh "node blockies-png.js {seed} {size}" "dir/to/save/testcase/files"
#
#   e.g 2) test only, don't generate test-io files (set output-dir as empty)
#    $ gen-rand-testcase-after-check.sh "node blockies-png.js {seed} {size}" "" 0 eth
#
#


COMPARE_CMD="${1?No web browser command}"
OUTPUT_FILE_DIR="$2"
INTERVAL="${3:-0}"
SEED_TYPE="$4"


GIT_ROOT="$(git rev-parse --show-toplevel)"
TEST_BIN="${GIT_ROOT}/target/release/eth-blockies"
GEN_TESTCASE_BASENAME="${GIT_ROOT}/tests/algorithm_validation/catimg-output-to-testcase"
GEN_TESTCASE_BIN="${GEN_TESTCASE_BASENAME}.bin"
GEN_TESTCASE_SRC="${GEN_TESTCASE_BASENAME}.rs"


# check eth-blockies bin
if ! command -v "$TEST_BIN" >/dev/null
then
  printf "No release binary found! (%s)\n" "$TEST_BIN" >&2
  exit 1
fi

# check escape-ascii bin
if ! command -v escape-ascii >/dev/null
then
  cargo install escape-ascii
  
  if ! command -v escape-ascii >/dev/null
  then
    printf "No escape-ascii binary found!\n" >&2
    exit 1
  fi
fi

# if write testcase outputs to file
if [ -n "$OUTPUT_FILE_DIR" ]
then
  # compile testcase generator if not exists
  if ! command -v "$GEN_TESTCASE_BIN" >/dev/null && ! rustc -O "$GEN_TESTCASE_SRC" -o "$GEN_TESTCASE_BIN"
  then
    printf "No test-io gen binary found! (%s)\n" "$GEN_TESTCASE_BIN" >&2
    exit 1
  fi

  # make new dir for output if not exists
  if ! [ -e "$OUTPUT_FILE_DIR" ]; then mkdir -p "$OUTPUT_FILE_DIR" || exit 1; fi
fi

# replace placeholder to variable in test command
COMPARE_CMD_REPLACED="$(printf "%s" "$COMPARE_CMD" | sed -e "s/{seed}/\"\$CUR_SEED\"/g" -e "s/{size}/\"\$CUR_BSIZE\"/g")"




##### start main loop #####
COUNT=0
while true
do
  COUNT=$((COUNT + 1))



  ### set input ###
  if [ "$SEED_TYPE" = "eth" ]
  then
    CUR_LEN=40
    CUR_SEED=0x"$(tr -dc '0-9a-f' < /dev/urandom | head -c"$CUR_LEN")"
    CUR_BSIZE=8
    RESULT_FNAME="$OUTPUT_FILE_DIR"/R"$CUR_BSIZE"_eth.txt
  else
    CUR_LEN="$(printf "%d" $(( ( $(head -c1 /dev/urandom | od -t u1 -An | tr -d '[:space:]') & 0x3F ) + 10 )) )"
    CUR_SEED="$(tr -dc '[:alpha:][:digit:]' < /dev/urandom | head -c"$CUR_LEN")"
    CUR_BSIZE="$(printf "%d" $(( ( $(head -c1 /dev/urandom | od -t u1 -An | tr -d '[:space:]') % 12 ) + 5 )) )"
    RESULT_FNAME="$OUTPUT_FILE_DIR"/R"$CUR_BSIZE".txt
  fi

  # if output dir defined, set dirname & basename. if not, reset fname var
  if [ -n "$OUTPUT_FILE_DIR" ]
  then
    RESULT_DNAME="$(basename "$(dirname "$RESULT_FNAME")")"
    RESULT_BNAME="$(basename "$RESULT_FNAME")"
  else
    RESULT_FNAME=
  fi

  # if file not exist, create one with header
  if [ -n "$RESULT_FNAME" ] && ! [ -e "$RESULT_FNAME" ]
  then
    printf "#\n# Auto-generated testcase file: '%s'\n#\n# format:\n#    [input] [output (<color0>,<color1>,<color2>=<indices-left-half>)]\n#\n" \
           "$RESULT_BNAME" > "$RESULT_FNAME"
  fi



  ### test commands ###
  TEST_OUTPUT="$("$TEST_BIN" "$CUR_SEED" -s "$CUR_BSIZE" -a)"
  EXTERNAL_TEST_OUTPUT="$(eval "$COMPARE_CMD_REPLACED" | catimg -w$(( CUR_BSIZE * 2)) -r1 -)"
  OUTPUTS_PASTED="$(for i in $(seq "$CUR_BSIZE"); do printf "\t"; printf "%s" "$TEST_OUTPUT" | cut -d'
' -f"$i" | tr -d '\n'; printf "\t"; printf "%s" "$EXTERNAL_TEST_OUTPUT" | cut -d'
' -f"$i" | tr -d '\n'; printf "\n"; done)"



  ### validate ###

  # reformat catimg output to fit eth-blockies-rs output before validation
  #  - remove extra header/footer '\x1b\[s\x1b\[\?25l', '\x1b\[\?25h'
  #  - remove extra '0;' right after the '\x1b\['
  #  - convert '\x1b\[m' to '\x1b\[0m'
  #  - pad R, G, B number values with 0
  EXTERNAL_TEST_OUTPUT_CONV="$(printf "%s" "$EXTERNAL_TEST_OUTPUT" | \
    sed -E \
        -e 's/^\x1b\[s\x1b\[\?25l//g' \
        -e 's/\x1b\[\?25h$//g' \
        -e 's/\x1b\[0;/\x1b\[/g' \
        -e 's/\x1b\[m/\x1b\[0m/g' \
        -e 's/\x1b\[([[:digit:]]+);([[:digit:]]+);([[:digit:]]+);([[:digit:]]+);([[:digit:]]+)m/\x1b\[\1;\2;000\3;000\4;000\5m/g' \
        -e 's/\x1b\[([[:digit:]]+);([[:digit:]]+);0+([[:digit:]]{3});0+([[:digit:]]{3});0+([[:digit:]]{3})m/\x1b\[\1;\2;\3;\4;\5m/g' \
    )"


  
  # check if the same
  if [ "$TEST_OUTPUT" = "$EXTERNAL_TEST_OUTPUT_CONV" ]
  then

    # if both outputs match
    printf "[ TEST # %d ]===================================================================================================\n   - LEFT:  $ %s\n   - RIGHT: $ %s\n%s\n   => MATCH! ( >> .../%s/%s )\n\n" \
      "$COUNT" \
      "$(basename "$TEST_BIN") \"$CUR_SEED\" -s \"$CUR_BSIZE\" -a" \
      "$(printf "%s" "$COMPARE_CMD" | sed -e "s/{seed}/\"$CUR_SEED\"/g" -e "s/{size}/\"$CUR_BSIZE\"/g")" \
      "$OUTPUTS_PASTED" \
      \
      "$RESULT_DNAME" "$RESULT_BNAME" >&2
    sleep "$INTERVAL"

    
    if [ -n "$RESULT_FNAME" ]
    then
      TEST_IO_OUTPUT="$(printf "%s" "$EXTERNAL_TEST_OUTPUT" | "$GEN_TESTCASE_BIN")" || exit 1
      printf "%s %s\n" "$CUR_SEED" "$TEST_IO_OUTPUT" | tee -a "$RESULT_FNAME" >/dev/null
    fi


    
  else
    
    # if both outputs don't match
    TEST_OUTPUT_COLORS="$(printf "%s" "$TEST_OUTPUT" | escape-ascii | sed -E -e 's/^\\x1b\[s\\x1b\[?25l//g' -e 's/\\x1b\[?25h$//g' -e 's/\\n/  /g' -e 's/\\x1b\[0m//g' -e 's/\\x1b\[([[:digit:]]+);([[:digit:]]+);([[:digit:]]+);([[:digit:]]+);([[:digit:]]+)m/(\3,\4,\5)/g' | tr -s ' ' | tr ' ' '\n' | sort -u | tr '\n' ' ')"
    TEST_EXTERNAL_OUTPUT_COLORS="$(printf "%s" "$EXTERNAL_TEST_OUTPUT_CONV" | escape-ascii | sed -E -e 's/^\\x1b\[s\\x1b\[?25l//g' -e 's/\\x1b\[?25h$//g' -e 's/\\n/  /g' -e 's/\\x1b\[0m//g' -e 's/\\x1b\[([[:digit:]]+);([[:digit:]]+);([[:digit:]]+);([[:digit:]]+);([[:digit:]]+)m/(\3,\4,\5)/g' | tr -s ' ' | tr ' ' '\n' | sort -u | tr '\n' ' ')"

    
    # if not the same, wait for user, and make user to compare directly
    printf "[ TEST # %d ]===================================================================================================\n   - LEFT:  $ %s\n   - RIGHT: $ %s\n%s\n   *** NOT MATCH: Press enter to add this seed to test io list! *** \n\n   * Color values of <eth-blockies>:        %s\n   * Color values of external test command: %s\n\n" \
      "$COUNT" \
      "$(basename "$TEST_BIN") \"$CUR_SEED\" -s \"$CUR_BSIZE\" -a" \
      "$(printf "%s" "$COMPARE_CMD" | sed -e "s/{seed}/\"$CUR_SEED\"/g" -e "s/{size}/\"$CUR_BSIZE\"/g")" \
      "$OUTPUTS_PASTED" \
      "$TEST_OUTPUT_COLORS" \
      "$TEST_EXTERNAL_OUTPUT_COLORS" >&2

    
    # wait user confirm (enter)
    read -r _ || exit 1
  fi

done
