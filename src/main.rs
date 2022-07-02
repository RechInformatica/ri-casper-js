use path_absolutize::Absolutize;
use serde_json::*;
use std::process::exit;
use std::{env, fs::read_link, path::Path};

/// CasperJS Wrapper Cross-platform
fn main() {

    // Supported engines by CasperJS
    let supported_engines = json!({
        "phantomjs" : {
            "native_args": [
                "cookies-file",
                "config",
                "debug",
                "disk-cache",
                "disk-cache-path",
                "ignore-ssl-errors",
                "load-images",
                "load-plugins",
                "local-url-access",
                "local-storage-path",
                "local-storage-quota",
                "offline-storage-path",
                "offline-storage-quota",
                "local-to-remote-url-access",
                "max-disk-cache-size",
                "output-encoding",
                "proxy",
                "proxy-auth",
                "proxy-type",
                "remote-debugger-port",
                "remote-debugger-autorun",
                "script-encoding",
                "script-language",
                "ssl-protocol",
                "ssl-ciphers",
                "ssl-certificates-path",
                "ssl-client-certificate-file",
                "ssl-client-key-file",
                "ssl-client-key-passphrase",
                "web-security",
                "webdriver",
                "webdriver-logfile",
                "webdriver-loglevel",
                "webdriver-selenium-grid-hub",
                "wd",
                "w",
            ],
            "env_varname": "PHANTOMJS_EXECUTABLE",
            "default_exec" : "phantomjs"
        },
        "slimerjs": {
            "native_args": [
                "-P",
                "-jsconsole",
                "-CreateProfile",
                "-profile",
                "error-log-file",
                "user-agent",
                "viewport-width",
                "viewport-height",
                "cookies-file",
                "config",
                "debug",
                "disk-cache",
                "ignore-ssl-errors",
                "load-images",
                "load-plugins",
                "local-storage-path",
                "local-storage-quota",
                "local-to-remote-url-access",
                "max-disk-cache-size",
                "output-encoding",
                "proxy",
                "proxy-auth",
                "proxy-type",
                "remote-debugger-port",
                "remote-debugger-autorun",
                "script-encoding",
                "ssl-protocol",
                "ssl-certificates-path",
                "web-security",
                "webdriver",
                "webdriver-logfile",
                "webdriver-loglevel",
                "webdriver-selenium-grid-hub",
                "wd",
                "w",
            ],
            "env_varname": "SLIMERJS_EXECUTABLE",
            "default_exec" : "slimerjs",
            "native_args_with_space": [
                "-P",
                "-CreateProfile",
                "-profile",
            ]
        },
    });

    // Get the execution info from environment variables
    let mut engine = env::var("CASPERJS_ENGINE").unwrap_or_else(|_| "phantomjs".to_string());
    let engine_flags = env::var("ENGINE_FLAGS").unwrap_or_default();
    let mut engine_args = shlex::split(engine_flags.as_str()).unwrap_or_default();

    // Get the localization of the current executable
    let current_exe = env::current_exe().unwrap_or_default();
    let path = current_exe.to_str().unwrap_or_default();

    // Solve CasperJS location
    let resolved = resolve(path.to_string());
    let dirname = Path::new(resolved.as_str())
        .parent()
        .unwrap()
        .join("..");

    // Get absolute path of CasperJS engine
    let absolutize = Absolutize::absolutize(&dirname).unwrap_or_default();
    let casper_path = absolutize.to_str().unwrap_or_default();

    // Get the executable args
    let sys_args: &Vec<String> = &env::args().collect();

    // Checks if a specific engine was informed from the command line
    for arg in sys_args {
        if let Some(e) = arg.strip_prefix("--engine=") {
            engine = e.to_string();
            break;
        };
    }

    // If the engine is not supported, finish the current execution
    if supported_engines.get(&engine) == None {
        exit(1);
    }

    // Get the native args of the engine
    let engine_native_args = supported_engines
        .get(&engine)
        .unwrap()
        .get("native_args")
        .unwrap();

    // Get the native args of the engine
    let engine_native_args_with_space = supported_engines
        .get(&engine)
        .unwrap()
        .get("native_args_with_space");

    // Get the location of engine executable
    let env_varname = supported_engines
        .get(&engine)
        .unwrap()
        .get("env_varname")
        .unwrap()
        .as_str()
        .unwrap();
    let mut engine_executable = env::var(&env_varname).unwrap_or_default();

    // If can't get the engine executable, verify if it was informed on environment variable
    // If cant't get from environment variables too, assumes a default information that is in the JSON of the supported engines
    if engine_executable.is_empty() {
        engine_executable = match env::var("ENGINE_EXECUTABLE") {
            Ok(var) => var,
            Err(_) => supported_engines
                .get(&engine)
                .unwrap()
                .get("default_exec")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
        };
    }

    let mut casper_args: Vec<String> = Vec::new();

    let mut iter = sys_args.iter();

    // Iterate the engine args
    while let Some(arg) = iter.next() {
        let arg_name = extract_arg_name(arg.clone());
        let mut found = false;
        if engine_native_args.get(&arg_name) != None {
            engine_args.push(arg.to_string());
            if let Some(x) = engine_native_args_with_space {
                if x.get(&arg_name) != None {
                    let next_arg = iter.next().unwrap();
                    if next_arg.is_empty() || next_arg.starts_with("--") {
                        exit(1);
                    } else {
                        engine_args.push(next_arg.to_string());
                    }
                }
            }
            found = true;
        }
        if !found && arg_name != "engine" {
            casper_args.push(arg.to_string());
        }
    }

    // Build the args list to the engine executable
    let mut casper_command = vec![engine_executable];
    casper_command.extend(engine_args);
    let path = Path::new(&casper_path)
        .join("bin")
        .join("bootstrap.js")
        .to_str()
        .unwrap()
        .to_string();
    let args_vec = vec![
        path,
        ["--casper-path=", &casper_path].concat(),
        "--cli".to_string(),
    ];
    casper_command.extend(args_vec);
    casper_args.remove(0);
    casper_command.extend(casper_args);

    // Replace the current proccess by the engine execution
    let _err = exec::execvp(casper_command.get(0).unwrap(), &casper_command);

    // Exit program with status code 1, because just will be here if the command fail
    exit(1);
}

fn resolve(path: String) -> String {
    let link_path = Path::new(path.as_str());
    return match read_link(link_path) {
        Ok(p) => Path::join(
            Path::new(&p.to_str().unwrap_or_default()).parent().unwrap(),
            read_link(&p).unwrap_or_default().to_str().unwrap_or_default().to_string(),
        )
        .to_str()
        .unwrap_or_default()
        .to_string(),
        Err(_) => path,
    };
}

fn extract_arg_name(arg: String) -> String {
    let result: Vec<&str> = arg.split('=').collect();
    result[0].replace("--", "")
}
