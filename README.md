# Instagram Followers 

Easy way to check follower stats on insta through exported followers /
following. Instagram makes you pay for it (I think), and 3rd party apps are sus
so this is an easy way to check if you're handy with rust.


# Assumptions 

I assume you already have a Rust toolchain installed and configured.


# Getting The Data

Bound to change, but here are the steps I took on the web:

1. -> Settings
2. -> See more in accounts center (Above edit profile)
3. -> Your information and permissions 
4. -> Export your information 
5. -> Create export 
6. -> Instagram 
7. -> Export to device 
8. Customize info -> select only followers and following 
9. Date range -> All time
10. Format -> JSON 
11. Export 
12. Wait a few minutes
13. unzip using your tool of choice (unzip on linux)
14. Move data files to current directory.



# Running 

I would recommend just running with cargo. Here is the help output:


```
~/Projects/followers master ❯ cargo run -- --help
Usage: followers [OPTIONS] --followers-file <FOLLOWERS_FILE> --following-file <FOLLOWING_FILE>

Options:
      --followers-file <FOLLOWERS_FILE>  The path to the followers file downloaded from insta
      --following-file <FOLLOWING_FILE>  The path to the following file downloaded from insta
  -m, --mutal                            Show mutual followers
  -t, --them-not-you                     Show people that you follow but not vice versa
  -n, --not-following-them               Show people follow you but not vice versa
  -h, --help                             Print help
```

For example, assuming your files are named `following.json` and
`followers_1.json` run the following command to get all output:


```shell
cargo run -- --followers-file followers_1.json --following-file following.json \
    -m -t -n
```

And it should print everything to the console.


# Notes 

Not sure if the JSON format is stable, may not work moving forward.
