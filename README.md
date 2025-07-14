## example

```
auth sufficient pam_sysauth.so --otp --env-log-filter debug
```

## Arguments

```
pam_sysauth.so  [[--otp]] [--otp-prompt STRING] [--password-prompt STRING] [( --logger-type-env | --logger-type-file )] [--env-log-filter STRING] [--file-log-path STRING] [--file-log-level STRING]

[[--otp]]
    shows otp. Default: absent

[--otp-prompt STRING]                        
    otp prompt. Default: "OTP: "

[--password-prompt STRING]                   
    password prompt. Default: "Password: " 

[( --logger-type-journald | --logger-type-env | --logger-type-file )]
    logger type: Journald, Environment, File. Default is Journald
 
[--env-log-filter STRING]                    
    rust log filter for Environment logger. Default: INFO

[--file-log-path STRING]                  
   file path for File logger. Default: /tmp/pam-sysauth.log
   
[--file-log-level STRING]  
   log level for File logger. Default: DEBUG

[--journald-log-level STRING]  
   log level for File logger. Default: INFO
```
