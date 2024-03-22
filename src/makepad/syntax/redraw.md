# Redrawing Mechanism

Understanding the correct strategies for redrawing is crucial for optimizing application performance and user experience in UI development. Here are two commonly used methods for redrawing and their appropriate scenarios.

## Immediate Redraw

The `apply_over_and_redraw` method allows for the immediate redrawing of the target component. This means that as soon as this method is executed, the specified component will be updated and redrawn right away.

**Applicable Scenario**: This method is suitable when immediate reflection of user interactions or data changes on the UI is required. It ensures the interface responds instantly.

**Example Code**:

```rust
impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(id!(btn1)).clicked(&actions) {
            self.counter += 1;
            let label = self.ui.label(id!(t_label));
            label.apply_over_and_redraw(cx, live! {
                text: (format!("Click Number: {}", self.counter)),
            });
        }
    }
}
```

## Batch Redraw

Another method involves using `apply_over` to update the data first, followed by a unified call to `redraw` for redrawing. This method allows for batching changes to multiple components or data, followed by a one-time UI update, reducing the number of renders.

**Applicable Scenario**: When updating multiple components or making multiple data changes, using batch updates followed by a one-time redraw can improve performance by avoiding unnecessary rendering overhead.

**Example Code**:

```rust
impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(id!(btn1)).clicked(&actions) {
            self.counter += 1;
            let label = self.ui.label(id!(t_label));
            label.apply_over(cx, live! {
                text: (format!("Click Number: {}", self.counter)),
            });
            label.redraw(cx);
        }
    }
}
```

## Considerations

- Using `apply_over_and_redraw` within a redraw function may lead to continuous redraws, which should be avoided in such contexts.
- Generally, it's recommended to use `apply_over` to update data where possible and then call `redraw` to redraw collectively, for optimizing application performance.

By carefully choosing the appropriate redraw strategy, you can ensure a good user experience while optimizing the application's performance.