use makepad_widgets::*;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 

    DesktopBtnExample = <SolidView>{
        height: 260,
        flow: Down,
        align: {x: 0.5, y: 0.5},
        spacing: 16,
        <DesktopButton> {draw_bg: {button_type: Fullscreen}}
        <DesktopButton> {draw_bg: {button_type: XRMode}}
        <DesktopButton> {draw_bg: {button_type: WindowsMin}}
        <DesktopButton> {draw_bg: {button_type: WindowsMax}}
        <DesktopButton> {draw_bg: {button_type: WindowsClose}}
        <DesktopButton> {draw_bg: {button_type: WindowsMaxToggled}}
    }
}
