<a align="center" name="readme-top"></a>




<!-- PROJECT LOGO -->
<br />
<div align="center">
  <h1 align="center">MakeReadMe</h1>

<!-- PROJECT SHIELDS -->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MITLicense][license-shield]][license-url]
  <p align="center">
    A CLI for creating a README for your github project
 
  </p>
</div>


<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project
This project aims to automate part of the coding workflow by helping you to quickly generate a good looking README. You answer the questions, we write the boilerplate for you.


<p align="right">(<a href="#readme-top">back to top</a>)</p>


### Built With
- [Clap](https://github.com/clap-rs/clap)
- [Inquire](https://github.com/mikaelmello/inquire)
- [Handlebars Rust](https://github.com/sunng87/handlebars-rust)
- [And more...](/Cargo.toml)


<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- GETTING STARTED -->
## Getting Started
  This is section will teach you how to set up and run the program

### Prerequisites
  For now, you will need *cargo* and *rust* installed on your system.
  ```sh
    curl https://sh.rustup.rs -sSf | sh 
  ```
  > Eventually, a non-cargo installation method will be provided.

### Installation
  Just cargo install the application and you are good to go.
````sh
  cargo install makereadme
``````

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- USAGE EXAMPLES -->
## Usage
  The README.md will be created on your current directory.
  Just run 
  ```sh
  makereadme
  ```
  Answer the prompts and your README.md will be ready for you to give some final touch.


<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- ROADMAP -->
## Roadmap
 - [ ] Full test coverage
 - [ ] Environment analysis and completions
 - [ ] Better performance and binary size
 - [ ] Better error handling
 - [ ] Documentation with Examples
 - [ ] More templating options and allow customization

See the [open issues](https://github.com/github_username/repo_name/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.
Take a look at [CODE OF CONDUCT](/CODE_OF_CONDUCT.md) and [CONTRIBUTING](/CONTRIBUTING.md) for more info.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- LICENSE -->
## License

Distributed under the MIT License. See [LICENSE](/LICENSE) for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- ACKNOWLEDGMENTS -->
## Acknowledgments
  - [readme-md-generator](https://github.com/kefranabg/readme-md-generator)
  - [Best-README-Template](https://github.com/othneildrew/Best-README-Template)
  - [ReadME-Generator](https://github.com/ShaanCoding/ReadME-Generator)


<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/caio-bernardo/MakeReadme.svg?style=for-the-badge  
[contributors-url]: https://github.com/caio-bernardo/MakeReadme/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/caio-bernardo/MakeReadme.svg?style=for-the-badge  
[forks-url]: https://github.com/caio-bernardo/MakeReadme/network/members
[stars-shield]: https://img.shields.io/github/stars/caio-bernardo/MakeReadme.svg?style=for-the-badge   
[stars-url]: https://github.com/caio-bernardo/MakeReadme/stargazers
[issues-shield]: https://img.shields.io/github/issues/caio-bernardo/MakeReadme.svg?style=for-the-badge   
[issues-url]: https://github.com/caio-bernardo/MakeReadme/issues
[license-shield]: https://img.shields.io/github/license/caio-bernardo/MakeReadme.svg?style=for-the-badge  
[license-url]: https://github.com/caio-bernardo/MakeReadme/blob/master/LICENSE
