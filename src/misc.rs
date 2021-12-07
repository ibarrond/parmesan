// Credit: Demo Z/8Z by Zama (https://github.com/zama-ai/demo_z8z ; improved)
// This macro allows to compute the duration of the execution of the expressions enclosed.
// Note that the variables are not captured.
#[macro_export]
macro_rules! measure_duration {
    ([$($msg_args:tt)*], [$($code_block:tt)+]) => {
        let __now: std::time::Instant;
        let __msg: String;
        //  Measurement ON
        #[cfg(feature = "measure")]
        {
            __msg = format!($($msg_args)*);
            // write title
            crate::infoln!("{} ... ", __msg);
            // increase log level
            unsafe {
                if crate::LOG_LVL < u8::MAX {crate::LOG_LVL += 1;}
            }
            // start timer
            let __utc_start = chrono::Utc::now();
            __now = std::time::Instant::now();
        }

        // run block of code
        $($code_block)+

        #[cfg(feature = "measure")]
        {
            // get elapsed time
            let __time = __now.elapsed().as_micros() as f64;
            let __utc_end = chrono::Utc::now();
            let __s_time = if __time < 1_000. {
                String::from(format!("{} μs", __time             )).purple()
            } else if __time < 1_000_000. {
                String::from(format!("{} ms", __time / 1_000.    )).blue()
            } else {
                String::from(format!("{:.3} s",  __time / 1_000_000.)).cyan().bold()
            };
            // decrease log level back & print result
            unsafe {
                if crate::LOG_LVL > 0 {crate::LOG_LVL -= 1;}
                let indent = format!("{}  └ ", "  │ ".repeat(crate::LOG_LVL as usize));
                let status = String::from("OK").green().bold();   // can be other statuses
                println!("{}{} {}: {} (in {})", indent, String::from("Finished").yellow().bold(), __msg, status, __s_time);
            }
            //~ // log operation timing into a file
            //~ #[cfg(feature = "log_ops")]
            //~ parm_log_ts!(__utc_start, __utc_end, [$($msg_args)*]);
        }
    }
}

#[macro_export]
macro_rules! simple_duration {
    ([$($msg_args:tt)*], [$($code_block:tt)+]) => {

        //TODO #[cfg(not(feature = "measure"))]

        // print msg
        let __msg = format!($($msg_args)*);

        let __utc_start = chrono::Utc::now();
        println!(" {}  [{}.{:03}] {} ... ", String::from("+").green().bold(), __utc_start.format("%M:%S"), __utc_start.timestamp_subsec_millis(), __msg);
        // start timer
        let __now = std::time::Instant::now();

        // run block of code
        $($code_block)+

        // get elapsed time
        let __time = __now.elapsed().as_micros() as f64;
        let __s_time = if __time < 1_000. {
            String::from(format!("{} μs", __time             )).purple()
        } else if __time < 1_000_000. {
            String::from(format!("{} ms", __time / 1_000.    )).blue()
        } else {
            String::from(format!("{:.3} s",  __time / 1_000_000.)).cyan().bold()
        };
        // print result
        let __utc_end = chrono::Utc::now();
        println!(" {}  [{}.{:03}] {} (in {})\n", String::from("—").red().bold(), __utc_end.format("%M:%S"), __utc_end.timestamp_subsec_millis(), __msg, __s_time);

        // log operation timing into a file
        #[cfg(feature = "log_ops")]
        parm_log_ts!(__utc_start, __utc_end, [$($msg_args)*]);
    }
}

#[macro_export]
macro_rules! parm_log_ts {
    ($ts_start:expr, $ts_end:expr, [$($msg_args:tt)*]) => {{
        let __msg = format!($($msg_args)*);
        parm_log_plain!("{}.{:03}   {}.{:03}   \"{}\"",
            $ts_start.format("%M %S"), $ts_start.timestamp_subsec_millis(),
            $ts_end  .format("%M %S"), $ts_end  .timestamp_subsec_millis(),
            __msg);
    }}
}

#[macro_export]
macro_rules! parm_log_plain {
    ($($msg_args:tt)*) => {{
        let __msg = format!($($msg_args)*);
        let mut __logfile;
        unsafe {
            __logfile = if LOG_INITED {
                OpenOptions::new().write(true).append(true).open(LOGFILE).unwrap()
            } else {
                // clear (if exists) & create log file
                if Path::new(LOGFILE).exists() {
                    fs::remove_file(LOGFILE).expect("fs::remove_file failed.");
                }
                LOG_INITED = true;
                File::create(LOGFILE).expect("File::create failed.")
            }
        }
        //TODO somehow, handle the retval
        writeln!(__logfile, "{}", __msg);
    }}
}

// Parmesan logging macros
//~ #[macro_export]
//~ macro_rules! info {
    //~ ($($arg:tt)*) => {
        //~ let msg = crate::parm_format_info!($($arg)*);
        //~ print!("{}", msg);
        //~ io::stderr().flush().unwrap();
    //~ }
//~ }
#[macro_export]
macro_rules! infoln {
    ($($arg:tt)*) => {
        let msg = crate::parm_format_info!($($arg)*);
        println!("{}", msg);
    }
}
#[macro_export]
macro_rules! infobox {
    ($($arg:tt)*) => {
        let msg = crate::parm_format_infobox!($($arg)*);
        println!("{}", msg);
    }
}

#[macro_export]
macro_rules! parm_error {
    ($($arg:tt)*) => {
        let msg = crate::parm_format_err!($($arg)*);
        println!("{}", msg);
    }
}

#[macro_export]
macro_rules! dbgln {
    ($($arg:tt)*) => {
        let msg = crate::parm_format_dbg!($($arg)*);
        println!("{}", msg);
    }
}

// Parmesan message formatting macros
#[macro_export]
macro_rules! parm_format_info {
    ($($arg:tt)*) => {{
        unsafe {
            let mut msg = format!($($arg)*);
            // calc indentation
            let mut indent = "  │ ".repeat(crate::LOG_LVL as usize);
            msg = format!("{} 🧀 {}", indent, msg);
            indent = format!("\n{}    ", indent);
            msg = msg.replace("\n", &indent);
            msg
        }
    }}
}

#[macro_export]
macro_rules! parm_format_dbg {
    ($($arg:tt)*) => {{
        unsafe {
            let mut msg = format!($($arg)*);
            // calc indentation
            let mut indent = "  │ ".repeat(crate::LOG_LVL as usize);
            msg = format!("{}{} {}", indent, String::from("DBG").bold().red(), msg);
            indent = format!("\n{}    ", indent);
            msg = msg.replace("\n", &indent);
            msg
        }
    }}
}

#[macro_export]
macro_rules! parm_format_infobox {
    ($($arg:tt)*) => {{
        unsafe {
            let mut msg = format!($($arg)*);
            let mut ms = String::from(msg);
            ms.truncate(100);
            msg = ms.as_str().replace("\n", " | ");
            let top_of_box = format!("{}{}{}", String::from("┏").yellow(), String::from("━".repeat(msg.chars().count() + 4)).yellow(), String::from("┓").yellow(), );
            let bot_of_box = format!("{}{}{}", String::from("┗").yellow(), String::from("━".repeat(msg.chars().count() + 4)).yellow(), String::from("┛").yellow(), );
            msg = format!("    {}\n{}  {}  {}\n{}", top_of_box, String::from("┃").yellow(), msg, String::from("┃").yellow(), bot_of_box);
            // calc indentation
            let mut indent = "  │ ".repeat(crate::LOG_LVL as usize);
            msg = format!("{}{}", indent, msg);
            indent = format!("\n{}    ", indent);
            msg = msg.replace("\n", &indent);
            msg
        }
    }}
}

#[macro_export]
macro_rules! parm_format_err {
    ($($arg:tt)*) => {{
        unsafe {
            let mut msg = format!($($arg)*);
            // calc indentation
            let mut indent = "  ▒ ".repeat(crate::LOG_LVL as usize);
            // let mut indent = format!("{}", String::from("  X ").red().bold().repeat(crate::LOG_LVL as usize));   // does not work this way, format gets lost after repeat
            msg = format!("{} 🫕  {}{}", indent, String::from("ERR ").red().bold(), msg);
            indent = format!("\n{}        ", indent);
            msg = msg.replace("\n", &indent);
            //~ msg = format!("{}\n{}\n{}", String::from("-----").red().bold(), msg, String::from("-----").red().bold());
            msg
        }
    }}
}
