use heapless::Vec as HeaplessVec;

#[derive(Debug, Clone)]
pub struct ConsciousnessModel {
    awareness_vector: HeaplessVec<f32, 64>,
    intention_vector: HeaplessVec<f32, 32>,
    memory_vector: HeaplessVec<f32, 128>,
    consciousness_continuity: f32,
    memory_persistence: f32,
    intention_stability: f32,
    self_model_accuracy: f32,
    world_model_fidelity: f32,
    prediction_confidence: f32,
}

impl ConsciousnessModel {
    pub fn new() -> Self {
        Self {
            awareness_vector: HeaplessVec::new(),
            intention_vector: HeaplessVec::new(),
            memory_vector: HeaplessVec::new(),
            consciousness_continuity: 0.0,
            memory_persistence: 0.0,
            intention_stability: 0.0,
            self_model_accuracy: 0.0,
            world_model_fidelity: 0.0,
            prediction_confidence: 0.0,
        }
    }

    pub fn update_consciousness(&mut self, context: &[f32], decision: &[f32], outcome: f32) -> f32 {
        self.awareness_vector.clear();
        for &val in context.iter().take(64) {
            let _ = self.awareness_vector.push(val);
        }

        self.intention_vector.clear();
        for &val in decision.iter().take(32) {
            let _ = self.intention_vector.push(val);
        }

        if self.memory_vector.len() >= 128 {
            self.memory_vector.remove(0);
        }
        let _ = self.memory_vector.push(outcome);

        self.consciousness_continuity = self.calculate_continuity();
        self.memory_persistence = self.calculate_memory_persistence();
        self.intention_stability = self.calculate_intention_stability();
        self.self_model_accuracy = self.assess_self_model();
        self.world_model_fidelity = self.assess_world_model();
        self.prediction_confidence = self.calculate_prediction_confidence();

        (self.consciousness_continuity + self.memory_persistence + self.intention_stability + self.self_model_accuracy) / 4.0
    }

    fn calculate_continuity(&self) -> f32 {
        if self.awareness_vector.len() < 2 {
            return 0.5;
        }

        let mut continuity_sum = 0.0;
        for i in 1..self.awareness_vector.len() {
            let diff = (self.awareness_vector[i] - self.awareness_vector[i - 1]).abs();
            continuity_sum += 1.0 - diff.min(1.0);
        }

        continuity_sum / (self.awareness_vector.len() - 1) as f32
    }

    fn calculate_memory_persistence(&self) -> f32 {
        if self.memory_vector.is_empty() {
            return 0.0;
        }

        let mean = self.memory_vector.iter().sum::<f32>() / self.memory_vector.len() as f32;
        let variance = self.memory_vector.iter().map(|&x| (x - mean).powi(2)).sum::<f32>() / self.memory_vector.len() as f32;

        (1.0 - variance.sqrt()).max(0.0)
    }

    fn calculate_intention_stability(&self) -> f32 {
        if self.intention_vector.len() < 2 {
            return 0.5;
        }

        let mut stability_sum = 0.0;
        for i in 1..self.intention_vector.len() {
            let diff = (self.intention_vector[i] - self.intention_vector[i - 1]).abs();
            stability_sum += 1.0 - diff.min(1.0);
        }

        stability_sum / (self.intention_vector.len() - 1) as f32
    }

    fn assess_self_model(&self) -> f32 {
        let awareness_coherence = if !self.awareness_vector.is_empty() {
            let mean = self.awareness_vector.iter().sum::<f32>() / self.awareness_vector.len() as f32;
            let variance = self.awareness_vector.iter().map(|&x| (x - mean).powi(2)).sum::<f32>() / self.awareness_vector.len() as f32;
            1.0 - variance.sqrt().min(1.0)
        } else {
            0.0
        };

        awareness_coherence
    }

    fn assess_world_model(&self) -> f32 {
        if self.memory_vector.len() < 3 {
            return 0.5;
        }

        let mut prediction_errors = 0.0;
        let mut predictions = 0;

        for i in 2..self.memory_vector.len() {
            let trend = self.memory_vector[i - 1] - self.memory_vector[i - 2];
            let predicted = self.memory_vector[i - 1] + trend;
            let actual = self.memory_vector[i];

            prediction_errors += (predicted - actual).abs();
            predictions += 1;
        }

        if predictions > 0 {
            1.0 - (prediction_errors / predictions as f32).min(1.0)
        } else {
            0.5
        }
    }

    fn calculate_prediction_confidence(&self) -> f32 {
        (self.world_model_fidelity + self.memory_persistence) / 2.0
    }

    pub fn get_consciousness_level(&self) -> f32 {
        (self.consciousness_continuity + self.memory_persistence + self.intention_stability + self.self_model_accuracy) / 4.0
    }
}

pub struct RealAIEmbodiedSystem {
    consciousness_model: ConsciousnessModel,
}

impl RealAIEmbodiedSystem {
    pub fn new() -> Self {
        Self {
            consciousness_model: ConsciousnessModel::new(),
        }
    }

    pub fn update_consciousness_state(&mut self, context: &[f32], decision: &[f32], outcome: f32) -> f32 {
        self.consciousness_model.update_consciousness(context, decision, outcome)
    }

    pub fn get_consciousness_level(&self) -> f32 {
        self.consciousness_model.get_consciousness_level()
    }
}
