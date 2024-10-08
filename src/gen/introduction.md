<divs style="display:flex;justify-content:center;">
<pre style="color:#FF6332">
     _/_/_/  _/_/_/_/  _/      _/  _/    _/  _/_/_/   
  _/        _/        _/_/    _/  _/    _/    _/      
 _/  _/_/  _/_/_/    _/  _/  _/  _/    _/    _/       
_/    _/  _/        _/    _/_/  _/    _/    _/        
 _/_/_/  _/_/_/_/  _/      _/    _/_/    _/_/_/       
</pre>
</divs>

# Introduction

GenUI是一个创新的SFP前端框架，使用Rust语言开发，最初受到Vue3和Makepad的启发。旨在帮助用户更有效地使用Rust编写前端项目。

## Why SFP

GenUI的SFP功能是指其可插拔设计，可以根据开发者的需求调整和使用不同的底层技术或框架，如Makepad或Slint。这种设计允许GenUI通过专门为这些底层框架设计的插件来扩展和自定义其功能，从而更好地适应各种开发场景。

1. 通过插件提供强大的能力
2. 不受固有底层限制
3. 同时也是一种挑战

## Advantage

- 多框架兼容性
- 灵活
- 可扩展
- 社区助力
- 尊重用户

## Target Group

- 前端开发人员和全栈开发人员
- 创新和实验项目的开发人员
- 开源社区
- 独立开发人员
- 中小型项目开发（现在）

## Example

- [gosim.org](https://gen.ipter.org/1/) | [Source Code](https://github.com/Privoce/GenUI/tree/main/examples/gen_ark_makepad)
- [shader genui](https://gen.ipter.org/3/) | [Source Code](https://github.com/Privoce/GenUI/tree/main/examples/gosim_example)

## Support Plugin

以下是当前GenUI支持的以及未来极有可能支持的底层

- [x] Makepad
- [ ] Zed
- [ ] Dioxus