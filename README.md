# Build and deploy
anchor build && anchor deploy

# Publish
anchor idl init --filepath target/idl/puppet_master.json EqP43dPi9EWyqBEm543a8QwZQV5WamWMDyCi7vousBuM

anchor idl upgrade --filepath target/idl/puppet_master.json EqP43dPi9EWyqBEm543a8QwZQV5WamWMDyCi7vousBuM
