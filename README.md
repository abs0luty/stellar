# Stellar 🎵

Stellar is a music programming language designed to help you create, play, and experiment with music effortlessly. Whether you're a beginner or an experienced coder, Stellar makes music programming fun and intuitive!

## 🚀 Installation

1. Clone the Stellar repository:
   ```bash
   git clone https://github.com/abs0luty/stellar
   cd stellar/
   ```

2. Install Stellar using Cargo:
   ```bash
   cargo install
   ```

## ✨ Features

### 🎶 Playing Simple Notes
Stellar allows you to play individual notes, chords, and more:

```
C4.play                # Play a single note
[C3, E3, G3].play      # Play a chord
Cmaj7.play             # Use built-in chord shortcuts
```

---

### 🎛️ Using Custom Samples and Synths
Use your own sounds and synths to create unique music:

```
# Use the synth for notes and chords
with synth: dsaw {
    [C3, E3, G3].play
    Cmaj7.play
}

# Load and play an audio sample
let mykick = sample("lib/kick2.mp3")
mykick.play
```

---

### 🔁 Reuse Code with Sequences
Organize your music with reusable sequences:

```
# Define a sequence
sequence test {
    Cmaj.play       
    Am.play         
    Fmaj.play       
    G7.play
}

# Play the sequence
test.play
```

---

### 🥁 Playing Sequences in Parallel
Layer your music by running sequences simultaneously on different channels:

```
# Set tempo and steps per second
bpm 120
sps 4

# Play a melody on one channel
with channel: 0 {
    repeat 4 {
        Cmaj.play       
        Am.play         
        Fmaj.play       
        G7.play
    }
}

# Add a drum beat on another channel
with channel: 1 {
    repeat 4 {
        kick.play
        wait 2
    }
}
```

---

Stellar is designed to spark your creativity—have fun making music! 🎶
