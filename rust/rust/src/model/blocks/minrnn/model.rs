use burn::prelude::*;
use burn::nn;
use super::mingru::{MinGru, MinGruConfig, MinGruState};
use super::minlstm::{MinLstm, MinLstmConfig, MinLstmState};

#[derive(Config, Debug, Copy)]
pub enum MinRnnType {
    MinGru,
    MinLstm,
}

#[derive(Config, Debug)]
pub struct MinRnnLMConfig {
    pub vocab_size: usize,
    pub embedding_dim: usize,
    pub num_layers: usize,
    #[config(default = "MinRnnType::MinGru")]
    pub rnn_type: MinRnnType,
    #[config(default = 2)]
    pub expansion_factor: usize,
}

#[derive(Module, Debug)]
pub enum MinRnnLayer<B: Backend> {
    Gru(MinGru<B>),
    Lstm(MinLstm<B>),
}

#[derive(Module, Debug)]
pub struct MinRnnLM<B: Backend> {
    pub embedding: nn::Embedding<B>,
    pub layers: Vec<MinRnnLayer<B>>,
    pub lm_head: nn::Linear<B>,
    pub embedding_dim: usize,
    pub vocab_size: usize,
}

impl MinRnnLMConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> MinRnnLM<B> {
        let embedding = nn::EmbeddingConfig::new(self.vocab_size, self.embedding_dim).init(device);
        
        let layers = (0..self.num_layers)
            .map(|_| {
                match self.rnn_type {
                    MinRnnType::MinGru => {
                        let cfg = MinGruConfig {
                            input_features: self.embedding_dim,
                            expansion_factor: self.expansion_factor,
                        };
                        MinRnnLayer::Gru(cfg.init(device))
                    }
                    MinRnnType::MinLstm => {
                        let cfg = MinLstmConfig {
                            input_features: self.embedding_dim,
                            expansion_factor: self.expansion_factor,
                        };
                        MinRnnLayer::Lstm(cfg.init(device))
                    }
                }
            })
            .collect();
            
        let lm_head = nn::LinearConfig::new(self.embedding_dim, self.vocab_size)
            .with_bias(false)
            .init(device);
            
        MinRnnLM {
            embedding,
            layers,
            lm_head,
            embedding_dim: self.embedding_dim,
            vocab_size: self.vocab_size,
        }
    }
}

#[derive(Clone, Debug)]
pub enum MinRnnState<B: Backend> {
    Gru(MinGruState<B>),
    Lstm(MinLstmState<B>),
}

#[derive(Clone, Debug)]
pub struct MinRnnLMState<B: Backend> {
    pub layer_states: Vec<MinRnnState<B>>,
}

impl<B: Backend> MinRnnLM<B> {
    pub fn init(config: &MinRnnLMConfig, device: &B::Device) -> Self {
        config.init(device)
    }

    pub fn forward(
        &self, 
        x: Tensor<B, 2, Int>, 
        state: Option<MinRnnLMState<B>>
    ) -> (Tensor<B, 3>, MinRnnLMState<B>) {
        let [_b, _s] = x.dims();
        let mut x = self.embedding.forward(x);
        
        let mut next_layer_states = Vec::with_capacity(self.layers.len());
        let current_states = state.map(|s| s.layer_states).unwrap_or_else(|| Vec::new());
        
        for (i, layer) in self.layers.iter().enumerate() {
            let layer_state = current_states.get(i).cloned();
            
            let (out, next_s) = match (layer, layer_state) {
                (MinRnnLayer::Gru(gru), Some(MinRnnState::Gru(s))) => {
                    let (o, ns) = gru.forward(x.clone(), Some(vec![s]));
                    (o, MinRnnState::Gru(ns[0].clone()))
                }
                (MinRnnLayer::Gru(gru), None) => {
                    let (o, ns) = gru.forward(x.clone(), None);
                    (o, MinRnnState::Gru(ns[0].clone()))
                }
                (MinRnnLayer::Lstm(lstm), Some(MinRnnState::Lstm(s))) => {
                    let (o, ns) = lstm.forward(x.clone(), Some(vec![s]));
                    (o, MinRnnState::Lstm(ns[0].clone()))
                }
                (MinRnnLayer::Lstm(lstm), None) => {
                    let (o, ns) = lstm.forward(x.clone(), None);
                    (o, MinRnnState::Lstm(ns[0].clone()))
                }
                _ => panic!("State/Layer type mismatch in MinRnnLM"),
            };

            x = x + out; 
            next_layer_states.push(next_s);
        }
        
        let logits = self.lm_head.forward(x);
        
        (logits, MinRnnLMState { layer_states: next_layer_states })
    }

    pub fn empty_state(&self, batch_size: usize, device: &B::Device) -> MinRnnLMState<B> {
        let mut layer_states = Vec::with_capacity(self.layers.len());
        let hidden_size = self.embedding_dim * 2; // Assuming expansion factor 2
        for layer in &self.layers {
            match layer {
                MinRnnLayer::Gru(_) => {
                    layer_states.push(MinRnnState::Gru(MinGruState::new(Tensor::zeros([batch_size, 1, hidden_size], device))));
                }
                MinRnnLayer::Lstm(_) => {
                    layer_states.push(MinRnnState::Lstm(MinLstmState::new(Tensor::zeros([batch_size, 1, hidden_size], device))));
                }
            }
        }
        MinRnnLMState { layer_states }
    }
}
