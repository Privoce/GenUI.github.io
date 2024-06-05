

use makepad_widgets::*;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 

    SlideExample = <SolidView>{
        height: 260,
        flow: Down,
        align: {x: 0.5, y: 0.5},
        spacing: 16,
        <SlidesView> {
            <Slide> {
                title = {text: " Hello Exec(ut)"},
                <SlideBody> {text: ""}
            }
            <Slide> {
                title = {text: "Playing with local AI"},
                <SlideBody> {text: "Rik Arends\n"}
            }
            <Slide> {
                title = {text:"Makepad"},
                <SlideBody> {text: "- Rebuilding VB/Delphi\n   in Rust"}
            }                    
            <Slide> {
                title = {text: "10 years is a long time"},
                <SlideBody> {text: "- Whole stack from scratch"}
            }
        }
    }
}


