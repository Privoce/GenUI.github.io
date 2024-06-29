# BuiltIn-UI Lib

Github Address: [gen makepad component lib](https://github.com/palpus-rs/Gen-UI/tree/main/gen/components)

--- 

|Version|Date|
|--|--|
|v0.0.1|2024-06-28|


## Feature

1. More diverse props(styles)
2. A simpler way of writing (more similar to CSS)
3. With default style, can be switched through themes
4. As a built-in component of GenUI, it can also be independently applied in Makepad projects
5. Continuous high-speed updates (currently updated approximately once a week)

## Import

### Toml

```toml
gen_components ={ git="https://github.com/palpus-rs/Gen-UI/tree/main/gen/components" }
```
### lib.rs

```rust
pub use gen_components;
```

### Live Design

```rust
live_design! {
    import gen_components::components::*;
}
```

## GenUI Builtin Lib


## Themes

each theme color has [25, 50, 100, 200, 300, 400, 500, 600, 700, 800, 900]

| Color Category | 25     | 50     | 100     | 200     | 300     | 400     | 500     | 600     | 700     | 800     | 900     |
|----------------|--------|--------|---------|---------|---------|---------|---------|---------|---------|---------|---------|
| Dark           | #FCFCFD| #F9FAFB| #F2F4F7 | #EAECF0 | #D0D5DD | #95A2D3 | #667085 | #475467 | #344054 | #1D2939 | #101828 |
| Primary        | #F5FEFF| #ECFDFF| #CFF9FE | #A5F0FC | #67E3F9 | #22CCEE | #06AED4 | #088AB2 | #0E6F90 | #155B75 | #164C63 |
| Error          | #FFFBFA| #FEF3F2| #FEE4E2 | #FECDCA | #FDA29B | #F97066 | #F04438 | #D92D2D | #B42318 | #912018 | #7A271A |
| Warning        | #FFFCF5| #FFFAEB| #FEF0C7 | #FEDF89 | #FEC84B | #FDB022 | #F79009 | #DC6803 | #B54708 | #93370D | #7A2E0E |
| Success        | #F6FEF9| #ECFDF3| #D1FADF | #A6F4C5 | #6CE9A6 | #32D583 | #12B76A | #039855 | #027A48 | #05603A | #054F31 |

---


<div style="overflow-x: scroll;">
<table cellspacing="0" border="0">
  <thead>
    <tr>
      <th>Color Category</th>
      <th>25</th>
      <th>50</th>
      <th>100</th>
      <th>200</th>
      <th>300</th>
      <th>400</th>
      <th>500</th>
      <th>600</th>
      <th>700</th>
      <th>800</th>
      <th>900</th>
    </tr>
  </thead>
  <tbody style="color:#FF7043;">
    <tr>
      <td>Dark</td>
      <td style="background:#FCFCFD;height:3em">#FCFCFD</td>
      <td style="background:#F9FAFB;height:3em">#F9FAFB</td>
      <td style="background:#F2F4F7;height:3em">#F2F4F7</td>
      <td style="background:#EAECF0;height:3em">#EAECF0</td>
      <td style="background:#D0D5DD;height:3em">#D0D5DD</td>
      <td style="background:#95A2D3;height:3em">#95A2D3</td>
      <td style="background:#667085;height:3em">#667085</td>
      <td style="background:#475467;height:3em">#475467</td>
      <td style="background:#344054;height:3em">#344054</td>
      <td style="background:#1D2939;height:3em">#1D2939</td>
      <td style="background:#101828;height:3em">#101828</td>
    </tr>
    <tr>
      <td>Primary</td>
      <td style="background:#F5FEFF;height:3em">#F5FEFF</td>
      <td style="background:#ECFDFF;height:3em">#ECFDFF</td>
      <td style="background:#CFF9FE;height:3em">#CFF9FE</td>
      <td style="background:#A5F0FC;height:3em">#A5F0FC</td>
      <td style="background:#67E3F9;height:3em">#67E3F9</td>
      <td style="background:#22CCEE;height:3em">#22CCEE</td>
      <td style="background:#06AED4;height:3em">#06AED4</td>
      <td style="background:#088AB2;height:3em">#088AB2</td>
      <td style="background:#0E6F90;height:3em">#0E6F90</td>
      <td style="background:#155B75;height:3em">#155B75</td>
      <td style="background:#164C63;height:3em">#164C63</td>
    </tr>
    <tr>
      <td>Error</td>
      <td style="background:#FFFBFA;height:3em">#FFFBFA</td>
      <td style="background:#FEF3F2;height:3em">#FEF3F2</td>
      <td style="background:#FEE4E2;height:3em">#FEE4E2</td>
      <td style="background:#FECDCA;height:3em">#FECDCA</td>
      <td style="background:#FDA29B;height:3em">#FDA29B</td>
      <td style="background:#F97066;height:3em">#F97066</td>
      <td style="background:#F04438;height:3em">#F04438</td>
      <td style="background:#D92D2D;height:3em">#D92D2D</td>
      <td style="background:#B42318;height:3em">#B42318</td>
      <td style="background:#912018;height:3em">#912018</td>
      <td style="background:#7A271A;height:3em">#7A271A</td>
    </tr>
    <tr>
      <td>Warning</td>
      <td style="background:#FFFCF5;height:3em">#FFFCF5</td>
      <td style="background:#FFFAEB;height:3em">#FFFAEB</td>
      <td style="background:#FEF0C7;height:3em">#FEF0C7</td>
      <td style="background:#FEDF89;height:3em">#FEDF89</td>
      <td style="background:#FEC84B;height:3em">#FEC84B</td>
      <td style="background:#FDB022;height:3em">#FDB022</td>
      <td style="background:#F79009;height:3em">#F79009</td>
      <td style="background:#DC6803;height:3em">#DC6803</td>
      <td style="background:#B54708;height:3em">#B54708</td>
      <td style="background:#93370D;height:3em">#93370D</td>
      <td style="background:#7A2E0E;height:3em">#7A2E0E</td>
    </tr>
    <tr>
      <td>Success</td>
      <td style="background:#F6FEF9;height:3em">#F6FEF9</td>
      <td style="background:#ECFDF3;height:3em">#ECFDF3</td>
      <td style="background:#D1FADF;height:3em">#D1FADF</td>
      <td style="background:#A6F4C5;height:3em">#A6F4C5</td>
      <td style="background:#6CE9A6;height:3em">#6CE9A6</td>
      <td style="background:#32D583;height:3em">#32D583</td>
      <td style="background:#12B76A;height:3em">#12B76A</td>
      <td style="background:#039855;height:3em">#039855</td>
      <td style="background:#027A48;height:3em">#027A48</td>
      <td style="background:#05603A;height:3em">#05603A</td>
      <td style="background:#054F31;height:3em">#054F31</td>
    </tr>
  </tbody>
</table>
</div>