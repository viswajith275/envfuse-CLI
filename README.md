**🔐 MASTER BRANCH**

```text
 /$$$$$$$$                       /$$$$$$                      /$$
| $$_____/                      /$$__  $$                    | $$
| $$       /$$$$$$$  /$$    /$$| $$  \__/  /$$$$$$   /$$$$$$ | $$
| $$$$$   | $$__  $$|  $$  /$$/|  $$$$$$  /$$__  $$ |____  $$| $$
| $$__/   | $$  \ $$ \  $$/$$/  \____  $$| $$$$$$$$  /$$$$$$$| $$
| $$      | $$  | $$  \  $$$/   /$$  \ $$| $$_____/ /$$__  $$| $$
| $$$$$$$$| $$  | $$   \  $/   |  $$$$$$/|  $$$$$$$|  $$$$$$$| $$
|________/|__/  |__/    \_/     \______/  \_______/ \_______/|__/
```

🚀 **EnvSeal**: Your Encrypted Personal Vault for Secrets

A lightweight, encrypted command-line vault for securely managing API keys, secrets, and sensitive environment variables on your local machine. EnvSeal organizes your secrets into **groups** (like `dev`, `staging`, `prod`) and keeps them under military-grade encryption. No more scattered `.env` files or accidental commits of sensitive data!

**Version:** see [Releases](https://github.com/viswajith275/EnvSeal-CLI/releases) for the current version and changelog.

---

## 🎯 Table of Contents

* [Why EnvSeal?](#why-envseal)
* [Problem and Solution](#problem-and-solution)
* [Key Features](#key-features)
* [Architecture & Security](#architecture--security)
* [Installation](#installation)
* [Quick Start](#quick-start)
* [Commands Reference](#commands-reference)
* [Usage Scenarios](#usage-scenarios)
  * [Local Development](#local-development-across-environments)
  * [CI/CD Pipeline Testing](#cicd-pipeline-local-testing)
  * [Server Deployment](#server-deployment)
* [Security Details](#security-details)
* [Best Practices](#best-practices)
* [Upcoming Features](#upcoming-features)
* [Troubleshooting](#troubleshooting)
* [Contributing](#contributing)
* [License](#license)

---

## 🎨 Why EnvSeal?

Imagine juggling multiple `.env` files across different projects, environments, and machines. You switch between local development, staging, and production—each with different API keys, database URLs, and credentials. One wrong copy-paste, and you've exposed production secrets to your dev environment. 😱

**EnvSeal eliminates this chaos** by:
- 🔒 **Centralizing** all your secrets in one encrypted vault
- 🚦 **Organizing** secrets by environment/project groups
- ⚡ **Injecting** secrets safely into child processes without polluting your shell
- 🎯 **Switching** between environments with a single command
- 🛡️ **Protecting** everything with AES-256-GCM encryption + Argon2 hashing
- 🧹 **Wiping** sensitive data from memory automatically

---

## 🚨 Problem and Solution

### The Problem

Developers often struggle with securely managing API keys, tokens, and sensitive credentials:

* **Plain-text storage disasters**: Keeping secrets in `.env` files or shell scripts exposes them to accidental commits and unauthorized access. A single `git add .` can turn into a security breach!
* **Environment sprawl**: Switching between different API keys for different environments (dev, staging, prod) is tedious and error-prone. Copy-paste mistakes can cause critical failures.
* **Global scope leaks**: Loading secrets into your global terminal session means any background script, sudo command, or spawned process can access them. Your entire session becomes a security liability.
* **Scattered secrets**: API keys scattered across multiple config files are hard to audit and manage. Where did that old Stripe key go?
* **No audit trail**: When secrets are compromised, you have no idea which process accessed them or when.

### The Solution

**EnvSeal provides**:

✅ **Group-Based Management**: Organize secrets into environments (e.g., `dev`, `prod`, `staging`) and switch seamlessly with a single command.

✅ **Secure Process Injection**: Use `envseal run` to inject secrets only into a specific child process. Your parent shell stays clean—no risk of global scope pollution.

✅ **End-to-End Encryption**: All sensitive data is encrypted at rest using **AES-256-GCM**, the same standard used by government agencies and major tech companies.

✅ **Master Password Protection**: The vault is locked behind a single, strong master password derived via **Argon2** (memory-hard hashing resistant to brute-force attacks).

✅ **Drop-in `.env` Replacement**: Easily import existing `.env` files in bulk or export groups back to `.env` format for CI/CD pipelines.

✅ **Memory Safety**: Built in Rust with automatic memory wiping via the `zeroize` crate to prevent secret leaks in RAM.

✅ **Cross-Platform**: Works seamlessly on Linux, macOS (Intel & Apple Silicon), with support for bash, zsh, and fish shells.

---

## 🌟 Key Features

| Feature | Description |
|---------|-------------|
| **🔐 AES-256-GCM Encryption** | Military-grade authenticated encryption with associated data (AEAD) |
| **🔑 Argon2 Key Derivation** | Memory-hard password hashing resistant to GPU/ASIC attacks |
| **📦 Group Organization** | Store secrets by project or environment (dev, staging, prod, etc.) |
| **🏃 Process Injection** | Safely run applications with decrypted variables injected directly—parent shell stays clean |
| **📥 Bulk Import/Export** | Parse `.env` files into encrypted groups in seconds |
| **💾 Persistent Vault** | Encrypted vault stored locally in your OS config directory |
| **🧹 Memory Safety** | Automatic memory wiping prevents sensitive data leaks |
| **🔄 Cross-Platform** | Linux, macOS (Intel & Apple Silicon), bash/zsh/fish |
| **📝 Directory Linking** (Upcoming) | Bind a group to a directory—no need to specify group name repeatedly |
| **🏷️ Dynamic Tags** (Upcoming) | Create dev/prod tags to swap variables on the fly |
| **💨 Zero-Overhead** | Written in Rust for speed and safety |

---

## 🏗️ Architecture & Security

### Encryption Pipeline

```
Master Password
       ↓
   Argon2 KDF (Memory-Hard Hashing)
       ↓
   256-bit AES Key
       ↓
   AES-256-GCM Encryption
       ↓
   Encrypted Vault (JSON)
```

### Secrets Storage

- **Location**: OS-specific config directory (`~/.config/envseal/` on Linux, `~/Library/Application Support/envseal/` on macOS)
- **Format**: JSON with encrypted payloads and random 12-byte nonces
- **Permissions**: File-level permissions + encryption = defense in depth

### Memory Safety

EnvSeal uses the Rust `zeroize` crate to:
- Overwrite sensitive data in memory immediately after use
- Prevent secrets from being accidentally dumped in core files
- Ensure no residual plaintext passwords remain in RAM

---

## 📦 Installation

Choose your preferred installation method.

### Method 1: Automated Installation (Recommended) 🎯

Download and run the installation script. The installer auto-detects your OS and architecture, installs the matching binary, and sets up automatic shell integration.

**Installation command:**

```bash
curl -sSfL https://raw.githubusercontent.com/viswajith275/EnvSeal-CLI/master/scripts/install.sh | bash
```

**Available options:**

```bash
./install.sh --help
```

| Option | Description |
|--------|-------------|
| `-v, --version <tag>` | Install a specific release (e.g., `v1.2.3`). Default: latest |
| `-d, --dir <path>` | Custom install directory. Default: `~/.local/bin` |
| `-f, --file <path>` | Install from a local tarball or binary |
| `--dry-run` | Preview changes without installing |

**Examples:**

```bash
# Install latest version
curl -sSfL https://raw.githubusercontent.com/viswajith275/EnvSeal-CLI/master/scripts/install.sh | bash

# Install specific version
./install.sh --version v1.2.3

# Install to custom directory
./install.sh --dir /usr/local/bin

# Dry run to preview
./install.sh --dry-run
```

**The installer will:**
- Detect your OS/arch and fetch the appropriate binary
- Copy the binary to your install directory with execute permissions
- Attempt to add the install directory to your PATH in shell config
- Install a shell wrapper so `envseal load` can be evaluated in your current shell
- Verify the installation and print next steps

### Method 2: Build from Source 🛠️

**Prerequisites:**
- [Rust 1.70+](https://www.rust-lang.org/tools/install)
- `cargo` package manager

```bash
# Clone the repository
git clone https://github.com/viswajith275/EnvSeal-CLI.git
cd EnvSeal-CLI

# Build in release mode
cargo build --release

# Install to your PATH
mkdir -p ~/.local/bin
cp target/release/envseal ~/.local/bin/envseal
chmod +x ~/.local/bin/envseal

# Add to PATH if not already there
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Verify
envseal --version
```

### Method 3: Download Pre-built Binary 📥

Visit the [Releases](https://github.com/viswajith275/EnvSeal-CLI/releases) page and download the binary for your platform:

- `envseal-linux-x86_64.tar.gz` — Linux x86_64
- `envseal-linux-aarch64.tar.gz` — Linux ARM64
- `envseal-macos-x86_64.tar.gz` — macOS Intel
- `envseal-macos-aarch64.tar.gz` — macOS Apple Silicon (M1/M2)

```bash
# Example: macOS Apple Silicon
tar -xzf envseal-macos-aarch64.tar.gz
chmod +x envseal
mv envseal ~/.local/bin/
```

---

## 🚀 Quick Start

### Step 1️⃣: Initialize Your Vault

Create an encrypted vault with a master password:

```bash
envseal init
```

This creates an encrypted vault file in your OS config directory and prompts you to set a strong master password.

**💡 Tips for a strong password:**
- Use 16+ characters mixing upper, lower, numbers, and symbols
- Avoid dictionary words and personal information
- Consider using a passphrase instead (e.g., "BlueSky!Coffee42#Moon")

### Step 2️⃣: Store Secrets in a Group

Store your development secrets in a group named `dev`:

```bash
envseal set --group dev DATABASE_URL
# Prompts for master password and the secret value
```

The group is created automatically on first use. Store multiple secrets:

```bash
envseal set --group dev API_KEY
envseal set --group dev STRIPE_SECRET_KEY
envseal set --group dev JWT_SECRET
```

### Step 3️⃣: Import an Existing `.env` File

Migrate a plain-text `.env` file into an encrypted group named `prod`:

```bash
envseal import --group prod ~/.env.production
```

This reads all key-value pairs and securely encrypts them in one go.

### Step 4️⃣: Run an Application Safely ✨

Inject your group's secrets directly into a process without leaking them to the terminal:

```bash
envseal run --group dev npm start
```

This is the **recommended workflow**. It:
1. Prompts for your master password once
2. Decrypts the variables
3. Executes `npm start` with those variables in its environment
4. Automatically wipes sensitive data from memory
5. Leaves your parent shell clean

Compare this to the old way:
```bash
# ❌ BAD: Secrets in your shell history and global session
source .env.dev
npm start

# ✅ GOOD: Secrets isolated to child process only
envseal run --group dev npm start
```

---

## 📚 Commands Reference

### `envseal init`

Initialize a new encrypted vault with a master password.

```bash
envseal init
```

**Creates:**
- Encrypted vault file in OS config directory
- Master password stored securely (Argon2-derived key)

**Output:**
```
Vault initialized successfully!
Master password is required to access your secrets.
```

---

### `envseal set <KEY>`

Set or update a value for a given key in a group. Creates the group if it doesn't exist.

```bash
# Add a secret to the 'dev' group
envseal set --group dev GITHUB_TOKEN

# Add to 'prod' group
envseal set --group prod DATABASE_URL

# With tag (for dynamic values)
envseal set --group myapp --tag prod API_ENDPOINT
```

**Interactive prompt:**
```
Enter value for GITHUB_TOKEN: ••••••••••••
```

---

### `envseal get <KEY>`

Retrieve a stored secret from a specific group.

```bash
envseal get --group dev GITHUB_TOKEN
# Output: ghp_xxx...
```

---

### `envseal import <PATH>`

Bulk import all key-value pairs from a `.env` file into an encrypted group.

```bash
# Import development secrets
envseal import --group dev /path/to/.env.dev

# Import production secrets
envseal import --group prod /path/to/.env.production
```

**Format:** Standard `.env` file format
```
DATABASE_URL=postgresql://user:pass@localhost/db
API_KEY=sk_live_xxx
STRIPE_SECRET_KEY=rk_live_yyy
```

---

### `envseal export [KEYS...]`

Decrypt and output an entire group (or specific keys) in `.env` format. Pipe to a file as needed.

```bash
# Export entire group to stdout
envseal export --group prod

# Export specific keys
envseal export --group prod DATABASE_URL API_KEY

# Export to file
envseal export --group prod > .env.production

# Export with piping
envseal export --group dev | grep DATABASE
```

**Output:**
```
DATABASE_URL=postgresql://user:pass@localhost/db
API_KEY=sk_live_xxx
STRIPE_SECRET_KEY=rk_live_yyy
```

---

### `envseal run <COMMAND>` ⭐

**(Recommended)** Load environment variables for a group and execute a command in a child process. Variables are **not** exposed to your parent terminal.

```bash
# Run Node.js app with dev secrets
envseal run --group dev npm start

# Run Python script with prod secrets
envseal run --group prod python app.py

# Run shell script with staging secrets
envseal run --group staging ./deploy.sh

# Run with multiple arguments
envseal run --group dev node server.js --port 3000
```

**Key advantages:**
- Secrets isolated to child process only
- Your shell history stays clean
- Multiple processes can have different secret sets
- Memory is automatically wiped after process exit

---

### `envseal load [KEYS...]`

Load specified keys (or entire group) into your current terminal environment.

**⚠️ Security note:** This loads secrets into your shell session. Use `run` whenever possible instead.

```bash
# Load entire group into current shell
eval "$(envseal load --group dev)"

# Load specific keys only
envseal load --group dev DATABASE_URL API_KEY

# After shell integration is installed
envseal load dev  # Same as above
```

**When to use:**
- Development environments where isolation is less critical
- Interactive debugging sessions
- When you need manual environment setup

---

### `envseal list [GROUP]`

List all keys currently stored in a specific group.

```bash
envseal list --group dev
# Output:
# - DATABASE_URL
# - API_KEY
# - JWT_SECRET
# - STRIPE_SECRET_KEY
```

---

### `envseal remove [KEY]`

Delete a specific key from a group, or delete the entire group if no key is specified.

```bash
# Remove a single key
envseal remove --group dev OLD_API_KEY

# Delete the entire group (with confirmation)
envseal remove --group dev
# Are you sure you want to remove group 'dev'? This is irreversible. [y/N]
```

**⚠️ Warning:** Deletions are permanent and cannot be undone. The encrypted vault does not maintain backups.

---

### `envseal link <GROUP>` (Upcoming)

Bind a specific group to your current working directory. Once linked, commands run in that folder automatically use the linked group.

```bash
# In your project directory, link to 'myapp' group
envseal link myapp

# Now any `envseal run` command in this directory uses 'myapp' by default
cd /path/to/project
envseal run npm start  # Uses 'myapp' group automatically
```

---

## 💻 Usage Scenarios

### Local Development Across Environments

Instead of swapping `.env` files manually or keeping multiple copies, organize them by environment:

```bash
# Initial setup: Import from your existing .env files
envseal import --group dev .env.dev
envseal import --group staging .env.staging
envseal import --group prod .env.prod

# List what you have
envseal list --group dev
envseal list --group staging
envseal list --group prod

# Work on local dev
envseal run --group dev npm start

# Test against staging database without changing code
envseal run --group staging npm start

# Preview production (read-only, be careful!)
envseal export --group prod
```

**Benefits:**
- No `.env` files in your git repo
- Easy switching between environments
- Each environment isolated and encrypted
- No accidental commits of secrets

---

### CI/CD Pipeline Local Testing

Test your deployment scripts locally without hardcoding secrets or using external secret managers:

```bash
# Store deployment credentials
envseal import --group deploy /path/to/aws-creds.env

# Test deployment script locally
envseal run --group deploy ./scripts/deploy_to_aws.sh

# Verify outputs without exposing secrets
envseal run --group deploy ./scripts/check_health.sh

# Run integration tests with test database
envseal import --group test-db test-database.env
envseal run --group test-db npm run test:integration
```

**Example deployment script:**
```bash
#!/bin/bash
set -e

# These env vars are automatically available in this process
echo "Deploying to AWS..."
aws s3 sync dist/ s3://$AWS_BUCKET_NAME/
aws cloudfront create-invalidation --distribution-id $DISTRIBUTION_ID --paths "/*"

echo "Deployment complete!"
```

---

### Server Deployment

Deploy applications on servers with environment-specific secrets:

```bash
# On your CI/CD server (e.g., GitHub Actions, GitLab CI)
# 1. Export secrets to a temporary .env file (only when needed)
envseal export --group prod > /tmp/.env.prod

# 2. Use in your deployment
docker run \
  --env-file /tmp/.env.prod \
  -v /var/www/app:/app \
  myapp:latest

# 3. Clean up
rm /tmp/.env.prod

# OR use directly with run command
envseal run --group prod docker-compose up -d

# Monitor with secrets injected
envseal run --group prod ./scripts/health_check.sh

# Restart application with new secrets
envseal run --group prod systemctl restart myapp
```

**Key advantages for servers:**
- No `.env` files sitting on disk
- Secrets only in memory during execution
- Easy to rotate credentials (just re-import to vault)
- Audit trail: logs show which process used secrets

---

## 🔒 Security Details

### Encryption Specification

| Component | Implementation | Details |
|-----------|-----------------|---------|
| **Cipher** | AES-256-GCM | NIST-approved, authenticated encryption with associated data (AEAD) |
| **Key Size** | 256-bit | Equivalent to 2^256 possible keys |
| **Mode** | GCM (Galois/Counter Mode) | Provides both confidentiality and authenticity |
| **Nonce** | 12-byte random | Generated fresh for each encryption |
| **Key Derivation** | Argon2 | Memory-hard hash resistant to brute-force and GPU attacks |

### Master Password Security

Your master password is hashed using **Argon2**:

```
Master Password
      ↓
Argon2 (with random salt)
      ↓
256-bit AES Key
      ↓
Used for AES-256-GCM encryption
```

**Argon2 parameters:**
- Time cost: 3 iterations
- Memory cost: 64MB
- Parallelism: 4 threads
- Resistance: GPU-resistant and ASIC-resistant

### Attack Resistance

| Attack | Mitigation |
|--------|-----------|
| **Brute-force** | Argon2 memory-hard hashing + 256-bit key space |
| **Dictionary attacks** | High entropy from strong passwords + Argon2 parameters |
| **Memory dumps** | Automatic memory wiping via `zeroize` crate |
| **Side-channel** | Constant-time operations in cryptographic functions |
| **Replay attacks** | Fresh nonce for each encryption |

---

## 🛡️ Best Practices

### ✅ DO

1. **Use `envseal run` by default**
   ```bash
   ✅ envseal run --group dev npm start
   ❌ eval "$(envseal load --group dev)" && npm start
   ```
   - Keeps secrets isolated to child process
   - Your shell history stays clean

2. **Use strong master passwords**
   - 16+ characters with mixed case, numbers, symbols
   - Use passphrases for easier memorization
   - Store in a password manager if needed

3. **Regularly rotate secrets**
   ```bash
   envseal set --group prod API_KEY  # Update with new key
   ```

4. **Back up your vault**
   ```bash
   cp ~/.config/envseal/seal-encrypted.json ~/backups/
   ```
   - Even though it's encrypted, keep a backup!
   - Store backup in a secure location (encrypted USB, cloud storage with encryption)

5. **Use different master passwords for different machines**
   - Compromised master password on one machine doesn't affect others

6. **Audit which environment you're using**
   ```bash
   envseal list --group dev
   envseal export --group dev | head  # Spot check
   ```

### ❌ DON'T

1. **Avoid loading secrets into shell permanently**
   ```bash
   ❌ eval "$(envseal load dev)"  # Now whole shell has access
   ✅ envseal run --group dev app  # Only app process has access
   ```

2. **Don't commit `.env` files to git**
   ```bash
   ❌ git add .env
   ✅ git add .env.example  # Template only
   ✅ Use EnvSeal instead
   ```

3. **Don't use the same master password everywhere**
   ```bash
   ❌ Same password for dev, staging, prod
   ✅ Different passwords per environment/machine
   ```

4. **Don't export secrets to temporary files carelessly**
   ```bash
   ❌ envseal export --group prod > /tmp/.env  # /tmp is world-readable
   ✅ envseal export --group prod | xargs -I {} sh -c 'export {}; ...'
   ```

5. **Don't forget to verify your vault is working**
   ```bash
   envseal list --group dev  # Spot check
   envseal get --group dev API_KEY  # Verify key exists
   ```

---

## 🎁 Upcoming Features

### Dynamic Tags 🏷️

Change crucial variables on the fly. Create local and prod tags to swap out specific values—like localhost vs. a production DB—while the rest of your group settings stay the same.

```bash
# Create a group with dev tag
envseal set --group myapp --tag dev DATABASE_URL postgresql://localhost/db

# Same group, prod tag
envseal set --group myapp --tag prod DATABASE_URL postgresql://prod-db.example.com/db

# Run with specific tag
envseal run --group myapp --tag dev npm start
envseal run --group myapp --tag prod npm start
```

**Use case:** Quickly toggle between local and cloud databases without maintaining separate groups.

### Directory Linking 📁

Bind a specific group to your current working directory. Once linked, you can run commands in that folder without needing to repeatedly type or remember the group name.

```bash
# In your project directory, link to 'frontend' group
cd ~/projects/my-app
envseal link frontend

# Now any command in this directory uses 'frontend' by default
envseal run npm start  # Uses 'frontend' group automatically

# Switch to staging
envseal link staging
envseal run npm start  # Uses 'staging' group automatically
```

**Use case:** Multi-service projects where each service needs different secrets.

---

## 🐛 Troubleshooting

### "Vault not found" Error

**Problem:** `envseal: error: no vault found — run 'envseal init' first`

**Solution:** Create your initial encrypted vault:
```bash
envseal init
```

---

### "No entry / group named X" Error

**Problem:** Trying to get, run, or list a group/key that doesn't exist.

**Solution:** Keys and groups are case-sensitive. Check what you have:
```bash
envseal list --group mygroup
# Check for typos in group name
```

---

### `envseal load` isn't setting environment variables

**Problem:** Running `envseal load dev` prints export statements but they don't take effect.

**Solution:** The automatic shell wrapper wasn't installed. Manual workaround:

```bash
eval "$(envseal load --group dev)"
```

Or reinstall with shell integration:
```bash
curl -sSfL https://raw.githubusercontent.com/viswajith275/EnvSeal-CLI/master/scripts/install.sh | bash
# Reload your shell
source ~/.bashrc  # or ~/.zshrc or ~/.config/fish/config.fish
```

---

### Wrong secrets being used in `envseal run`

**Problem:** I ran `envseal run dev npm start` but the wrong group was used.

**Solution:** Verify which group you're actually running:
```bash
# Check what's in each group
envseal list --group dev
envseal list --group prod

# Export to confirm contents
envseal export --group dev | head
```

Always specify `--group` explicitly:
```bash
envseal run --group prod npm start  # Be explicit, not implicit
```

---

### "Permission denied" when installing

**Problem:** `install.sh: Permission denied`

**Solution:** Make the script executable:
```bash
chmod +x scripts/install.sh
./scripts/install.sh
```

Or run it directly:
```bash
bash scripts/install.sh
```

---

### Performance issues with large `.env` files

**Problem:** `envseal import` is slow with thousands of variables.

**Solution:** EnvSeal is optimized for typical use cases (< 1000 secrets). For massive imports:

1. Split your `.env` file:
   ```bash
   head -500 huge.env > huge.env.1
   tail -n +501 huge.env > huge.env.2
   ```

2. Import in batches:
   ```bash
   envseal import --group myapp huge.env.1
   envseal import --group myapp huge.env.2  # Appends to group
   ```

3. Or use multiple groups:
   ```bash
   envseal import --group app-core core.env
   envseal import --group app-services services.env
   ```

---

## 🤝 Contributing

Contributions are welcome! We're looking for:
- Bug fixes and error handling improvements
- Platform-specific testing (macOS M1, Linux ARM64, etc.)
- New features and optimizations
- Documentation improvements
- Security audits

### Development Setup

```bash
# Clone the repository
git clone https://github.com/viswajith275/EnvSeal-CLI.git
cd EnvSeal-CLI

# Build and test
cargo build
cargo test
cargo fmt        # Format code
cargo clippy     # Lint checks

# Run the binary locally
./target/debug/envseal --help
```

### Making Changes

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature`
3. Commit your changes: `git commit -am 'Add your feature'`
4. Push to the branch: `git push origin feature/your-feature`
5. Open a Pull Request with a clear description

### Code Standards

- Follow Rust conventions (use `cargo fmt`)
- Run `cargo clippy` to catch common mistakes
- Write tests for new functionality
- Update documentation for user-facing changes

---

## 📄 License

This project is licensed under the **MIT License** — see the [LICENSE](./LICENSE) file for details.

**In summary:** You're free to use, modify, and distribute EnvSeal for any purpose (personal, commercial, etc.). Just include the original license notice.

---

## 🙋 Support & Questions

- 📖 Check the [Troubleshooting](#troubleshooting) section
- 🐛 [Open an issue](https://github.com/viswajith275/EnvSeal-CLI/issues) on GitHub
- 💬 Discussions & feature requests welcome!

---

**Made with 🔒 and ❤️ for developers who value security.**

---

## 🏆 What Makes EnvSeal Different?

| Feature | EnvSeal | `.env` files | 1Password | AWS Secrets Manager |
|---------|---------|-------------|-----------|------------------|
| **Local-First** | ✅ | ✅ | ❌ Cloud | ❌ Cloud |
| **Encryption** | ✅ AES-256-GCM | ❌ Plain text | ✅ | ✅ |
| **Easy Setup** | ✅ | ✅ | ⚠️ Complex | ⚠️ Complex |
| **Cost** | 🆓 Free | 🆓 Free | 💰 ~$35/mo | 💰 Variable |
| **CLI-First** | ✅ | ✅ | ⚠️ UI focus | ✅ |
| **Process Isolation** | ✅ | ❌ | ⚠️ | ⚠️ |
| **Offline Support** | ✅ | ✅ | ❌ | ❌ |
| **Self-Contained** | ✅ | ✅ | ❌ | ❌ |

EnvSeal is perfect for developers who want:
- 🔒 Strong security without cloud dependencies
- ⚡ Quick setup and fast iteration
- 💰 No subscription costs
- 🎯 Isolated, containerized secret injection
- 🛠️ CLI-native workflow

---

**Ready to seal your secrets? Get started with `envseal init`! 🚀**
