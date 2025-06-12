# Rust API client for Grist

The client was originally intended to use the OpenAPI specification, but it turned out [specification had some issues](https://github.com/gristlabs/grist-help/issues/501) and was unable to generate the code.
I was able to fix the issues that prevented code generation, got it to compile, cleaned up the code a little, and got everything working for [Grist Image Optimizer](https://github.com/QazCetelic/grist-image-optimizer), but some other parts might still have issues.  
If you would like to interact with Grist in your Rust code and find any issues for your own projects, please create a PR with the fixed code.
