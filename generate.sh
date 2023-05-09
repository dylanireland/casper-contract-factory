casper-client put-deploy \
--node-address http://188.40.47.161:7777/rpc \
--chain-name casper-test \
--secret-key ./keys/secret_key.pem \
--payment-amount 50000000000 \
--session-hash hash-f7764049ddd5a8e3810ba60fa345b98f9c177d6b9bdab461509854d698d2487b \
--session-entry-point "generate"
