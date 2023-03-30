# Change Github Repos Naming
Change all of your repos(including private) from snake case to kebab case and vice-versa, e.g. "my_repo" to "my-repo". Currently the script only supports users but it could support organisations too.  

Created this after flip-flopping a couple times on which one I preferred. I think kebab case is usually better for URLs and package names but snake case is better for nearly everything else(directory names, code, etc.), and decided I wanted all my repos, Rust crate packages etc., to just be snake case for simplicity.  

## Usage
Clone this repo: `git clone https://github.com/mark-ruddy/change_github_repos_naming`

Set the `GITHUB_TOKEN` env var to your token which has access to updating repos: `export GITHUB_TOKEN=ghp_fVd...`

Pass in your username when running the script and the desired format, options are `kebab-to-snake` or `snake-to-kebab`:

```
*[main][~/dev/default/change_github_repos_naming]$ cargo run -- -u mark-ruddy --format kebab-to-snake
Finished dev [unoptimized + debuginfo] target(s) in 0.04s
Running `target/debug/change_github_repos_naming -u mark-ruddy --format kebab-to-snake`
token is: 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
Skipping repo UX, no change after formatting
Renaming repo algo-open-source-verifier to algo_open_source_verifier, continue? [y/n] y
Successfully renamed repo algo-open-source-verifier to algo_open_source_verifier
Renaming repo angular-tour to angular_tour, continue? [y/n] y
Successfully renamed repo angular-tour to angular_tour
Renaming repo argon-hash-password to argon_hash_password, continue? [y/n] y
Successfully renamed repo argon-hash-password to argon_hash_password
Skipping repo AstroNvim, no change after formatting
Skipping repo audio_converter, no change after formatting
```
