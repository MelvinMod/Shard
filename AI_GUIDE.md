# AI & Machine Learning in Shard

## Overview

Shard provides comprehensive AI/ML support including:

- ONNX model loading and inference
- LLM support (compatible with LM Studio, llama.cpp)
- Computer vision (object detection, classification)
- Transformer models (BERT, GPT, etc.)
- Neural network training

## ONNX Support

### Loading ONNX Models

```shard
use ai

let session = OnnxSession.new("model.onnx")
let input = [1.0, 2.0, 3.0, 4.0]
let output = session.predict(input)
```

### ONNX for Classification

```shard
use ai

fn classify_image(image_path: String, model_path: String)
  let session = OnnxSession.new(model_path)
  let image_data = load_image(image_path)
  let output = session.predict(image_data)
  return get_top_class(output)
~
```

## LLM Support

### Text Generation

```shard
use ai

let llm = LLM.new("llama-2-7b.gguf")
let prompt = "What is machine learning?"
let response = llm.generate(prompt, max_tokens=200)
say response
```

### Chat with LLM

```shard
use ai

let llm = LLM.new("mistral.gguf")

fn chat(conversation_history: Array[String])
  let prompt = join(conversation_history, "\n")
  return llm.generate(prompt, max_tokens=500)
~

let history = ["Human: Hello", "Assistant: Hi there!"]
let response = chat(history)
say response
```

### LM Studio Compatibility

```shard
use ai

# Load model from LM Studio directory
let llm = LLM.new("~/.cache/lm-studio/models/llama-2.gguf")
let response = llm.generate("Explain quantum computing", 300)
```

## Computer Vision

### Object Detection

```shard
use ai

let cv = ComputerVision.new("yolov8.onnx")
let objects = cv.detect_objects("photo.jpg")

each obj in objects
  say "Class: #{obj.class_name}"
  say "Confidence: #{obj.confidence}"
  say "Bounding Box: #{obj.bbox}"
~
```

### Image Classification

```shard
use ai

let cv = ComputerVision.new("resnet50.onnx")
let results = cv.classify("image.jpg")

# Get top 5 predictions
each (class, confidence) in results.take(5)
  say "#{class}: #{confidence * 100}%"
~
```

### Face Detection

```shard
use ai

let cv = ComputerVision.new("face_detector.onnx")
let faces = cv.detect_objects("group.jpg")

say "Found #{faces.len} faces"
```

## Transformer Models

### BERT Embeddings

```shard
use ai

let transformer = Transformer.new("bert-base-uncased")
let text = "Hello, how are you?"
let embedding = transformer.encode(text)

say "Embedding size: #{embedding.len}"
```

### Text Classification

```shard
use ai

let transformer = Transformer.new("distilbert-base")
let text = "I love this product!"
let embedding = transformer.encode(text)

# Classify sentiment
let sentiment = classify_sentiment(embedding)
say "Sentiment: #{sentiment}"
```

### Text Generation

```shard
use ai

let transformer = Transformer.new("gpt2")
let prompt = "Once upon a time"
let generated = transformer.generate(prompt, max_length=100)
say generated
```

## Neural Networks

### Simple Network

```shard
use ai

let nn = NeuralNetwork.new()
  .add_layer(Dense(784, 128, activation="relu"))
  .add_layer(Dense(128, 64, activation="relu"))
  .add_layer(Dense(64, 10, activation="softmax"))

nn.compile(optimizer="adam", loss="cross_entropy")
```

### Training

```shard
use ai

let nn = NeuralNetwork.new()
  .add_layer(Dense(784, 256, activation="relu"))
  .add_layer(Dense(256, 10, activation="softmax"))

nn.compile(optimizer=Adam(learning_rate=0.001), loss=CategoricalCrossentropy())

# Train
nn.fit(train_images, train_labels, epochs=50, batch_size=32)

# Evaluate
let accuracy = nn.evaluate(test_images, test_labels)
say "Accuracy: #{accuracy}"
```

### CNN for Images

```shard
use ai

let cnn = CNN.new()
  .add_layer(Conv2D(32, (3, 3), activation="relu", input_shape=(28, 28, 1)))
  .add_layer(MaxPooling2D((2, 2)))
  .add_layer(Conv2D(64, (3, 3), activation="relu"))
  .add_layer(MaxPooling2D((2, 2)))
  .add_layer(Flatten())
  .add_layer(Dense(128, activation="relu"))
  .add_layer(Dense(10, activation="softmax"))

cnn.compile(optimizer=Adam(0.001), loss=CategoricalCrossentropy())
cnn.fit(dataset, epochs=100)
```

## Complete AI Pipeline

```shard
use ai

fn create_image_classifier
  # Load pre-trained model
  let cv = ComputerVision.new("efficientnet.onnx")
  
  # Process image
  let image = load_image("photo.jpg")
  let resized = image.resize(224, 224)
  let normalized = resized.normalize()
  
  # Get predictions
  let results = cv.classify(normalized)
  
  # Display top 3
  each (class, confidence) in results.take(3)
    say "#{class}: #{confidence * 100}%"
  ~
~

create_image_classifier()
```

## Saving and Loading Models

```shard
use ai

# Save model
let nn = NeuralNetwork.new()
  .add_layer(Dense(100, 50))
  .add_layer(Dense(50, 10))

nn.save("my_model.onnx")

# Load model
let loaded_nn = OnnxSession.new("my_model.onnx")
let prediction = loaded_nn.predict(input_data)
```

## Performance Tips

1. **Batch Processing** - Process multiple inputs at once
2. **GPU Acceleration** - Use CUDA-enabled ONNX Runtime
3. **Model Quantization** - Reduce model size for faster inference
4. **Caching** - Cache embeddings and intermediate results

## Available Models

### Pre-trained Models
- BERT (text understanding)
- GPT (text generation)
- ResNet (image classification)
- YOLO (object detection)
- Stable Diffusion (image generation)

### Custom Models
- Train your own models
- Import from TensorFlow/PyTorch
- Export to ONNX format

---

**Author:** MelvinSGjr (MelvinMod)
