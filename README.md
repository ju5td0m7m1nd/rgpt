<a name="readme-top"></a>

<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]

<!-- PROJECT LOGO -->
<br />
<div align="center">
<img src="https://user-images.githubusercontent.com/12392676/233793965-fb55b753-ee8f-4eb4-acca-7011b9996ac5.png" alt="image" width="120" height="120">
<h3 align="center">RGPT</h3>

  <p align="center">
   The RGPT tool allows you to interact with OpenAI's GPT API through the command-line interface.<br/>
   Featuring a pre-configured prompt, it enables users to directly engage with the API from their terminal.
    <!-- <br />
    <a href="https://github.com/ju5td0m7m1nd/rgpt"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/ju5td0m7m1nd/rgpt">View Demo</a>
    ·
    <a href="https://github.com/ju5td0m7m1nd/rgpt/issues">Report Bug</a>
    ·
    <a href="https://github.com/ju5td0m7m1nd/rgpt/issues">Request Feature</a> -->
  </p>
</div>



https://user-images.githubusercontent.com/12392676/233795487-e5e58da4-6a9f-4f75-9217-525ed0c35dae.mp4




<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>

<br />
<br />

<!-- GETTING STARTED -->
## Getting Started

Configure your shorthand commands in the **settings.json** file.
And then run the `rgpt` command with the shorthand command and the text you want to send to the API.


### Prerequisites

* Rust
  [Install Rust](https://www.rust-lang.org/tools/install)
* OpenAI Api Key
  [Get OpenAI API Key](https://openai.com/)
  

### Installation

1. Set the OpenAI Api Key in **.env**
   ```env
   OPENAI_API_KEY={YOUR_KEY_HERE}  
   ```
2. Configure the prompt and key shortcut in **settings.json**
   ```sh
   mv .settings.example.com settings.json
   ```
3. Build the rust cli application
   ```sh
   cargo build --release
   ```
4. Configure `rgpt` as a global command in the terminal (In ~/.bashrc or ~/.zshrc)
   ```sh
   # Add the line to enable `rgpt` command in the terminal
   alias rgpt="{YOUR_PATH_TO_RGPT_REPO}/rgpt/target/release/rgpt"
   ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- USAGE EXAMPLES -->
## Usage

```sh
  rgpt gc "This is a example of there is some grammar error in it"

  # This will trigger the "gc" shorthand from the settings.json
  # settings.json { "gc": { prompt: "Correct the following text for me:"}}

  # The output will be:
  "This is an example of there being some grammar error in it."
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the MIT License. See [here][license-url] for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

[@mhtsai_95](https://twitter.com/mhtsai_95) 

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/ju5td0m7m1nd/rgpt.svg?style=for-the-badge
[contributors-url]: https://github.com/ju5td0m7m1nd/rgpt/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/ju5td0m7m1nd/rgpt.svg?style=for-the-badge
[forks-url]: https://github.com/ju5td0m7m1nd/rgpt/network/members
[stars-shield]: https://img.shields.io/github/stars/ju5td0m7m1nd/rgpt.svg?style=for-the-badge
[stars-url]: https://github.com/ju5td0m7m1nd/rgpt/stargazers
[issues-shield]: https://img.shields.io/github/issues/ju5td0m7m1nd/rgpt.svg?style=for-the-badge
[issues-url]: https://github.com/ju5td0m7m1nd/rgpt/issues
[license-shield]: https://img.shields.io/github/license/ju5td0m7m1nd/rgpt.svg?style=for-the-badge
[license-url]: https://github.com/ju5td0m7m1nd/rgpt/blob/main/LICENSE
[Rust-url]: https://www.rust-lang.org/
[OpenAI-url]: https://openai.com/product/
[product-screenshot]: images/screenshot.png
