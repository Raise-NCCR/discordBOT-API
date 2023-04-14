# discordBOT-API
Discord のボット用APIをNixとRustを用いて作ろうということでやっているもの  
  
ビルド方法  
・Nix のインストール  
https://nixos.org/download.html を参照  
`sh <(curl -L https://nixos.org/nix/install) --daemon`  
  
・direnv, nix-direnv のインストール・セットアップ  
`nix-env -iA nixpkgs.direnv`  
`nix-env -f '<nixpkgs>' -iA nix-direnv`  
`echo 'eval "$(direnv hook bash)"' >> ~/.bashrc`  
`source ~/.bashrc`  
`direnv allow`  
  
・実行  
`cargo run`  

