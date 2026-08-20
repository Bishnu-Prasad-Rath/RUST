#!/usr/bin/env node

const { spawn, spawnSync } = require('child_process');
const path = require('path');

const exePath = path.join(__dirname, 'sentinel.exe');
const args = process.argv.slice(2);

if (args[0] === 'start') {
  // Only detach if we are starting the background daemon
  const child = spawn(exePath, args, {
    detached: true,
    stdio: 'ignore' // Fully detach it from the terminal
  });
  
  // Unref tells Node it can close without waiting for this child
  child.unref(); 
  console.log("🚀 Daemon launched in the background!");
  process.exit(0);
  
} else {
  // For everything else (status, stop, stage), run normally so we can see the text
  const result = spawnSync(exePath, args, { stdio: 'inherit' });
  process.exit(result.status ?? 0);
}