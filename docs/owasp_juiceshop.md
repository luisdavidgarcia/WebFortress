To install Node.js version 20.x on your Ubuntu machine, you can follow these steps:

### Step 1: Update the Package Index
First, update your package index to ensure you have the latest information on the newest versions of packages and their dependencies.

```sh
sudo apt-get update
```

### Step 2: Install Node.js
To install Node.js 20.x, you need to use the NodeSource binary distributions. NodeSource provides an installer script that will add the NodeSource repository to your system and install Node.js.

#### Method 1: Using NodeSource Setup Script

1. **Download and run the NodeSource setup script for Node.js 20.x:**

```sh
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
```

2. **Install Node.js:**

```sh
sudo apt-get install -y nodejs
```

#### Method 2: Manual Setup

If you prefer to add the NodeSource repository manually and then install Node.js, follow these steps:

1. **Add the NodeSource APT repository:**

```sh
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
```

2. **Install Node.js:**

```sh
sudo apt-get install -y nodejs
```

### Step 3: Verify the Installation
After the installation is complete, verify that Node.js and npm (Node Package Manager) are correctly installed by checking their versions.

```sh
node -v
npm -v
```

You should see the version numbers of Node.js and npm, confirming that they are installed.

### Optional: Install `n` for Node Version Management

If you want to easily switch between different versions of Node.js, you can use `n`, a Node.js version manager. Here’s how to install and use `n`:

1. **Install `n` using npm:**

```sh
sudo npm install -g n
```

2. **Install the desired Node.js version using `n`:**

```sh
sudo n 20
```

3. **Set the desired Node.js version as the default:**

```sh
sudo n stable
```

### Summary
By following these steps, you can install Node.js version 20.x on your Ubuntu machine. Using the NodeSource setup script is the recommended and simplest way to ensure you get the correct version. Optionally, using `n` can help you manage multiple versions of Node.js on your system.