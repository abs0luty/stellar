<table style="border-collapse: collapse; border: none;">
<tr>
<td width="30%" valign="top">
<img src="./stellar.png" width="50%">
</td>
<td valign="top">
<b>Stellar</b> is a music programming language designed to help you create, play, and experiment with music effortlessly. Whether you're a beginner or an experienced coder, Stellar makes music programming fun and intuitive!
</td>
</table>

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

```python
play c4                # Play a single note
play [c3, e3, g3]      # Play a chord
play cmaj7             # Use built-in chord shortcuts
```

---

### ⏰ Add Pauses

```python
play c4
wait 1   # wait for one tact
play a4
```

---

### 🎛️ Using Custom Samples and Synths
Use your own sounds and synths to create unique music:

```python
# Use the synth for notes and chords
with synth: dsaw {
    play [c3, e3, g3]
    play cmaj7
}

# Load and play an audio sample
let mykick = sample("lib/kick2.mp3")
play mykick
```

---

### 🔁 Reuse Code with Sequences
Organize your music with reusable sequences:

```python
# Define a sequence
sequence test {
    play cmaj
    play am
    play fmaj
    play g7
}

# Play the sequence
play test
```

---

### 🥁 Playing Sequences in Parallel
Layer your music by running sequences simultaneously on different channels:

```python
# Set tempo 
set_bpm 120

# Play a melody on one channel
sequence melody {
    repeat 4 {
        play cmaj
        wait 1
        play am
        wait 1
        play fmaj
        wait 1
        play g7
    }
}

# Add a drum beat on another channel
sequence drum {
    repeat 4 {
        play kick
        wait 2
    }
}

play! melody # spans a new channel for playing the sequence
play drum
```

---

Stellar is designed to spark your creativity - have fun making music! 🎶