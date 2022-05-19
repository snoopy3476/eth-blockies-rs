use std::{collections::HashMap, env, io};

pub type OptList = HashMap<String, Option<String>>;
pub type ArgList = Vec<String>;

pub trait CmdOpt {
    // try to get option
    //
    // return:
    //   - if successful:  Ok( (is_option_set(exist), option_value_only_if_exist) )
    //   - if parse error: Err(io::Error)
    fn get_opt(
        &mut self,
        opt_name: &str,
        need_opt_value: bool,
    ) -> io::Result<(bool, Option<String>)>;

    // if unread argument exists, return error with invalid argument error msg
    fn check_if_empty(&self) -> io::Result<()>;
}

impl CmdOpt for OptList {
    fn get_opt(
        &mut self,
        opt_name: &str,
        need_opt_value: bool,
    ) -> io::Result<(bool, Option<String>)> {
        self.remove(opt_name) // pop as full opt name
            .or(opt_name.get(..1).and_then(|c| self.remove(c))) // pop as first char
            .map_or(
                // if opt does not exist, unset exist flag
                Ok((false, None)),
                // if opt exists, set exist flag
                |v| {
                    // check if value exists if needed, or does not exist if not needed
                    (v.is_some() == need_opt_value)
                        .then(|| (true, v))
                        .ok_or(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            format!(
                                "Invalid argument: Argument '{}' {} option value!",
                                opt_name,
                                match need_opt_value {
                                    true => "needs an",
                                    false => "must not have any",
                                }
                            ),
                        ))
                },
            )
    }

    fn check_if_empty(&self) -> io::Result<()> {
        match self.iter().next() {
            Some(remaining_opt) => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Invalid option: '{}'", remaining_opt.0),
            )),
            _ => Ok(()),
        }
    }
}

// parse args iterator, and return tuple of (option list, normal arg list)
pub fn parse_args() -> io::Result<(ArgList, OptList)> {
    let mut args = env::args();
    let _bin_name = args.next();

    args.fold(
        Ok(((ArgList::new(), OptList::new()), None, false)),
        |list_wrapper, arg| {
            list_wrapper.and_then(|(mut list, mut last_opt_name, opt_mode)| {
                // parse current argument
                // (branch according to: if OPTION/NORMAL ARG)
                match (
                    arg.to_string(),
                    arg.split_once('-')
                        .filter(|s| s.0.is_empty() && (!s.1.is_empty())),
                ) {
                    // OPTION ARG (starting with '-')
                    (_, Some(optstr)) => {
                        // check if long or short option argument
                        // (branch according to: if option argument is long/short form)
                        match (
                            optstr.1.to_string(),
                            optstr.1.split_once('-').filter(|s| {
                                s.0.is_empty() && {
                                    // s.1 is not empty && not '='
                                    s.1.chars().next().filter(|c| *c != '=').is_some()
                                }
                            }),
                        ) {
                            // long option
                            (_, Some(("", longopt))) => match longopt.find('=') {
                                // if opt value exists (embeded with '=')
                                Some(pos) => {
                                    longopt
                                        .get(..pos)
                                        .zip(longopt.get((pos + 1)..))
                                        // return (k, v)
                                        .map(|(k, v)| (k.to_string(), Some(v.to_string())))
                                }

                                // if opt value does not exist
                                _ => {
                                    // set next arg as value of current opt
                                    last_opt_name = Some(longopt.to_string());

                                    // return (k, v)
                                    Some((longopt.to_string(), None))
                                }
                            },

                            // short option
                            (shortopt, _) => match shortopt.split_at(1) {
                                // if opt value exists (embedded after option char)
                                (k, "") => {
                                    // set next arg as value of current opt
                                    last_opt_name = Some(shortopt.to_string());

                                    // return (k, v)
                                    Some((k.to_string(), None))
                                }

                                // if opt value does not exist
                                (k, v) => {
                                    // return (k, v)
                                    Some((k.to_string(), Some(v.to_string())))
                                }
                            },
                        }
                        // insert parsed key & value
                        .map(|(k, v)| list.1.insert(k, v))
                        .or_else(|| panic!("Unexpected error on parsing arguments!"));

                        // return next list_wrapper
                        Ok((list, last_opt_name, true))
                    }

                    // NORMAL ARG
                    (fullstr, _) => {
                        // set current string as opt value of last_opt_name
                        last_opt_name.as_ref().and_then(|opt_name| {
                            list.1.insert(opt_name.clone(), Some(fullstr.clone()))
                        });

                        // set current string as arg list, only if not in opt_mode
                        match opt_mode {
                            true => {
                                last_opt_name.ok_or(io::Error::new(
                                    io::ErrorKind::InvalidInput,
                                    format!("Invalid argument: '{}'", fullstr),
                                ))?;
                            }
                            false => {
                                list.0.push(fullstr.clone());
                            }
                        }

                        // return next list_wrapper
                        Ok((list, None, opt_mode))
                    }
                }
            })
        },
    )
    .map(|(list, _, _)| list)
}
