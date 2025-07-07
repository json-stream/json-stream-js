# Packaging TODO

1. **Add metadata for npm publication**
   - Update `package.json` with description, keywords, repository URL, author, and license fields.
   - Ensure `main` points to `index.js` which loads the compiled binary.

2. **Introduce a build system for prebuilt binaries**
   - Use `@napi-rs/cli` or a similar tool to compile the Rust code into Node prebuilds.
   - Configure builds for Node 16, 18, and 20 on Linux, macOS, and Windows.

3. **Add NPM packaging configuration**
   - Include generated prebuilt binaries via `.npmignore` or `files` in `package.json`.
   - Add appropriate `engines`, `os`, and `cpu` entries if needed.

4. **Create a CI/CD pipeline**
   - Use GitHub Actions to build across operating systems and Node versions.
   - Steps should install the Rust toolchain, run `npx napi build --platform`, and execute tests.

5. **Publish steps**
   - Automate version bumps and npm publishing triggered by tags.
   - Upload prebuilt binaries to releases or include them in the published package.

6. **Generate and configure NPM_TOKEN**
   - Create an `NPM_TOKEN` in the npm account and save it as a repository secret.
   - Reference this secret in the workflow to authenticate `npm publish`.

7. **Push package to npm on version bump**
   - Extend the CI/CD workflow to run `npm publish` using the token when the version is bumped.
   - Publish with `--access public` if the package is public.

8. **Verification**
   - After publishing, install the package on Linux, macOS, and Windows without build tools to confirm the binary loads.
   - Document supported platforms and Node versions in `README.md`.

9. **Optional enhancements**
   - Add a `prepublishOnly` script for tests and builds.
   - Consider using semantic-release for automated versioning and changelog generation.

