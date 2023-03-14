// Import Polkadot.js API dependencies.
const { decodeAddress, encodeAddress } = require('@polkadot/keyring')
const { hexToU8a, isHex } = require('@polkadot/util')
const { exec } = require('node:child_process');

async function run(){
    for (let index = 0; index < 512; index++) {
        address = "0x"+index.toString(16).padStart(64,0)
        address = encodeAddress(hexToU8a(address))
    
        let args = `cargo contract call --contract 5FHKCqCpykVpaceaJaLi2fqCeDcCjpqMXRtG3a8wy1hRXYCa --message add_candidate --args ${address} --suri //Alice --skip-confirm`;
        let call = exec(args, {cwd:"./.."})
        await new Promise((resolve,reject)=>{
            call.on('close', (code) => {
                console.log(`${index} child process exited with code ${code}`);
                resolve()
            });
            call.on('error', (err) => {
                console.error('Failed to start subprocess.',err);
                reject()
            });
        
            /*call.stdout.on('data', (data) => {
                console.log(data);
            });
            
            call.stderr.on('data', (data) => {
                console.error(`ps stderr: ${data}`);
            });*/
        })
    }
}
run()