#!/usr/bin/env node

/**
 * Sync version from package.json to:
 * - src-tauri/tauri.conf.json
 * - src-tauri/Cargo.toml
 * - src-tauri/gen/apple/project.yml
 * 
 * This script ensures package.json is the single source of truth for version numbers.
 */

const fs = require('fs');
const path = require('path');

// Read version from package.json
const packageJsonPath = path.join(__dirname, '..', 'package.json');
const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));
const version = packageJson.version;

if (!version) {
  console.error('Error: No version found in package.json');
  process.exit(1);
}

console.log(`Syncing version ${version} to configuration files...`);

// Update tauri.conf.json
const tauriConfPath = path.join(__dirname, '..', 'src-tauri', 'tauri.conf.json');
const tauriConf = JSON.parse(fs.readFileSync(tauriConfPath, 'utf8'));
tauriConf.version = version;
fs.writeFileSync(tauriConfPath, JSON.stringify(tauriConf, null, 2) + '\n');
console.log(`✓ Updated ${path.relative(process.cwd(), tauriConfPath)}`);

// Update Cargo.toml
const cargoTomlPath = path.join(__dirname, '..', 'src-tauri', 'Cargo.toml');
let cargoToml = fs.readFileSync(cargoTomlPath, 'utf8');
cargoToml = cargoToml.replace(/^version = ".*"$/m, `version = "${version}"`);
fs.writeFileSync(cargoTomlPath, cargoToml);
console.log(`✓ Updated ${path.relative(process.cwd(), cargoTomlPath)}`);

// Update project.yml
const projectYmlPath = path.join(__dirname, '..', 'src-tauri', 'gen', 'apple', 'project.yml');
let projectYml = fs.readFileSync(projectYmlPath, 'utf8');
// Replace CFBundleShortVersionString
projectYml = projectYml.replace(/CFBundleShortVersionString: .*$/m, `CFBundleShortVersionString: ${version}`);
// Replace CFBundleVersion (keep as string in quotes for consistency)
projectYml = projectYml.replace(/CFBundleVersion: ".*"$/m, `CFBundleVersion: "${version}"`);
fs.writeFileSync(projectYmlPath, projectYml);
console.log(`✓ Updated ${path.relative(process.cwd(), projectYmlPath)}`);

console.log(`\n✅ Successfully synced version ${version} to all configuration files!`);
