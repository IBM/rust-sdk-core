# Guidelines For Contributing
Thank you for your interest in contributing to our project. Whether it's a bug report, new feature, correction, or additional documentation, we greatly value feedback and contributions from our community.

Please read through this document before submitting any issues or pull requests to ensure we have all the necessary information to effectively respond to your bug report or contribution.

## Reporting Bugs/Feature Requests
We welcome you to use the GitHub issue tracker to report bugs or suggest features.

- Remember that we are still at the beginning of development, so most of the features are still under discussion and receiving new inputs.
- Search the [open issue](https://github.com/IBM/ibm-cloud-sdk/issues) and [closed issue](https://github.com/IBM/ibm-cloud-sdk/issues?q=is%3Aissue+is%3Aclosed) to ensure no one else has reported something similar before.
- Test with the latest master by building the JAR locally to see if the issue has already been addressed.
- You can also make a suggestion or ask a question by opening an "issue".
- File an [issue ticket](https://github.com/IBM/ibm-cloud-sdk/issues/new) by providing all the required information. Failure to provide enough detail may result in slow response from the community. Please try to include as much information as you can. Details like these are incredibly useful:
  * A reproducible test case or series of steps
  * The version of our code being used
  * Any modifications you've made relevant to the bug
  * Anything unusual about your environment or deployment

## Contributing via Pull Requests
Contributions via pull requests are much appreciated. Before sending us a pull request, please ensure that:

- Remember, much of the code was generated automatically by the OpenApi generator, to generate low-level code, but the SDK should make it easier for developers to use it.
- Search the [open issue](https://github.com/IBM/ibm-cloud-sdk/issues) to ensure no one else has reported something similar and no one is actively working on similar proposed change.
- If no one has suggested something similar, open an ["issue"](https://github.com/IBM/ibm-cloud-sdk/issues) with your suggestion to gather feedback from the community.
- If you are adding support to a new service from IBM Cloud, consider creating a new issue, and open a discussion to get feedback from the community.
- It's recommended to **create a new git branch** for the change so that the merge commit message looks nicer in the commit history.

## How to contribute
To send us a pull request, please:

1. Fork the repository.
2. Modify the source; please focus on the specific change you are contributing. If you also reformat all the code, it will be hard for us to focus on your change.
3. Ensure local tests pass.
4. Commit to your fork using clear commit messages.
5. Send us a pull request, answering any default questions in the pull request interface.
6. Pay attention to any automated CI failures reported in the pull request, and stay involved in the conversation.

GitHub provides additional document on [forking a repository](https://help.github.com/articles/fork-a-repo/) and
[creating a pull request](https://help.github.com/articles/creating-a-pull-request/).

## Finding contributions to work on
Looking at the existing issues is a great way to find something to contribute on. As our projects, by default, use the default GitHub issue labels (enhancement/bug/duplicate/help wanted/invalid/question/wontfix), looking at any 'help wanted' issues is a great place to start.


### Branches

Please file the pull request against the correct branch, e.g. `master` for non-breaking changes. See the [Git Branches](https://github.com/IBM/ibm-cloud-sdk/wiki/Git-Branches) page for more information.


### Style guide
Code change should conform to the programming style guide of the respective languages:

- Rust: https://github.com/rust-lang-nursery/fmt-rfcs/blob/master/guide/guide.md (the default [rustfmt](https://github.com/rust-lang-nursery/rustfmt) configuration)

You may find the current code base not 100% conform to the coding style and we welcome contributions to fix those.
Remember, much of the code was generated automatically by the OpenApi generator.

### Testing

Under COnstruction

### Tips
- Smaller changes are easier to review
- Add test case(s) to cover the change
- Document the fix in the code to make the code more readable
- Make sure test cases passed after the change (one way is to leverage https://travis-ci.org/ to run the CI tests)
- File a PR with meaningful title, description and commit messages
- Make sure the option "Allow edits from maintainers" in the PR is selected so that the maintainers can update your PRs with minor fixes, if needed.
- Recommended git settings
    - `git config core.autocrlf input` to tell Git convert CRLF to LF on commit but not the other way around
- To close an issue (e.g. issue 1542) automatically after a PR is merged, use keywords "fix", "close", "resolve" in the PR description, e.g. `fix #1542`. (Ref: [closing issues using keywords](https://help.github.com/articles/closing-issues-using-keywords/))