# Diff Notes

<p align="center">
<a href="https://github.com/lucaslrodri/diff-notes/releases/latest" target="_blank" rel="noopener noreferrer"
  ><img
      alt="Logo of Diff Notes"
      src="https://raw.githubusercontent.com/lucaslrodri/diff-notes/refs/heads/main/assets/app-icon.png"
      style="width: 6rem; max-width: calc(100%-2rem); max-height: 6rem"
  /></a>
<p align="center">
  <em>A truly minimal annotation tool with integrated differentiation and merge functionality</em>
</p>
<p align="center">
  <a href="https://github.com/lucaslrodri/diff-notes/releases/latest" target="_blank" rel="noopener noreferrer"
    ><img
        alt="GitHub release (latest by date)"
        src="https://img.shields.io/github/v/release/lucaslrodri/diff-notes?style=flat-square&logo=github&label=Latest%20Release"
        style="max-width: calc(100%-2rem)"
    /></a>
</p>

![App Screenshot](https://raw.githubusercontent.com/lucaslrodri/diff-notes/refs/heads/main/assets/screenshot.png)

> Diff Notes is a lightweight and intuitive annotation differencing and merging tool for Windows, built using Tauri and Rust. It enables you to compare two pieces of text side-by-side, highlighting the differences in a clear and visual format. Whether you're reviewing edits, comparing document versions, or analyzing changes made by AI tools like ChatGPT, Diff Notes provides a simple yet powerful solution.
>
> Each text is displayed in its own view, with dedicated controls for each view. This design gives you flexibility and precision when working with your content, making it ideal for tasks that require detailed text comparison.

## Getting Started

1. Download and install Diff Notes for Windows [here](https://github.com/lucaslrodri/diff-notes/releases/latest). 
    - Linux version is in the works.
2. Open the app and paste or type your text into the two views.
3. Use the controls to manage and compare your text.

## Features

- **Side-by-Side Comparison**: Compare two pieces of text in a split view.
- **Custom Titles**: Add custom titles for each text view for better organization.
- **Auto-Save and Load**: Automatically save and reload text content when the app is closed and reopened.
- **Flexible Layouts**: Switch between vertical and horizontal view layouts.
- **Chunked Comparison**: Compare large texts more effectively by chunking:
  - **Semantic Approach**: Uses Google's diff-match-patch algorithm for meaningful comparisons.
  - **Word-by-Word Approach**: Highlights differences at the word level.
- **Dedicated View Controls**:
  - Copy the full text of a view to the clipboard.
  - Overwrite a view's text with clipboard content.
  - Clone the text from one view to the other.
  - Approve or reject changes from the other view.

## 🚧 Work In Progress

This is an alpha version, so it may contain bugs and lacks comprehensive documentation. Your feedback and contributions are highly appreciated!

## License

This project is licensed under the [MIT License](LICENSE).