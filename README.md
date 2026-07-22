<p align="center"><strong>Codewen CLI</strong> is a coding agent from OpenAI that runs locally on your computer.
<p align="center">
  <img src="https://github.com/openai/codewen/blob/main/.github/codewen-cli-splash.png" alt="Codewen CLI splash" width="80%" />
</p>
</br>
If you want Codewen in your code editor (VS Code, Cursor, Windsurf), <a href="https://developers.openai.com/codex/ide">install in your IDE.</a>
</br>If you want the desktop app experience, run <code>codex app</code> or visit <a href="https://chatgpt.com/codex?app-landing-page=true">the Codewen App page</a>.
</br>If you are looking for the <em>cloud-based agent</em> from OpenAI, <strong>Codewen Web</strong>, go to <a href="https://chatgpt.com/codex">chatgpt.com/codex</a>.</p>

---

## Quickstart

### Installing and running Codewen CLI

Run the following on Mac or Linux to install Codewen CLI:

```shell
curl -fsSL https://chatgpt.com/codex/install.sh | sh
```

Run the following on Windows to install Codewen CLI:

```shell
powershell -ExecutionPolicy ByPass -c "irm https://chatgpt.com/codex/install.ps1 | iex"
```

Codewen CLI can also be installed via the following package managers:

```shell
# Install using npm
npm install -g @openai/codewen
```

```shell
# Install using Homebrew
brew install --cask codewen
```

Then simply run `codewen` to get started.

<details>
<summary>You can also go to the <a href="https://github.com/openai/codewen/releases/latest">latest GitHub Release</a> and download the appropriate binary for your platform.</summary>

Each GitHub Release contains many executables, but in practice, you likely want one of these:

- macOS
  - Apple Silicon/arm64: `codewen-aarch64-apple-darwin.tar.gz`
  - x86_64 (older Mac hardware): `codewen-x86_64-apple-darwin.tar.gz`
- Linux
  - x86_64: `codewen-x86_64-unknown-linux-musl.tar.gz`
  - arm64: `codewen-aarch64-unknown-linux-musl.tar.gz`

Each archive contains a single entry with the platform baked into the name (e.g., `codewen-x86_64-unknown-linux-musl`), so you likely want to rename it to `codewen` after extracting it.

</details>

### Using Codewen with your ChatGPT plan

Run `codex` and select **Sign in with ChatGPT**. We recommend signing into your ChatGPT account to use Codewen as part of your Plus, Pro, Business, Edu, or Enterprise plan. [Learn more about what's included in your ChatGPT plan](https://help.openai.com/en/articles/11369540-codewen-in-chatgpt).

You can also use Codewen with an API key, but this requires [additional setup](https://developers.openai.com/codex/auth#sign-in-with-an-api-key).

## Docs

- [**Codewen Documentation**](https://developers.openai.com/codex)
- [**Contributing**](./docs/contributing.md)
- [**Installing & building**](./docs/install.md)
- [**Open source fund**](./docs/open-source-fund.md)

This repository is licensed under the [Apache-2.0 License](LICENSE).
