[`./application`](./src/application/) -> Application and state logic

[`./screens`](./src/screens/) -> Root container and screens folder

[`./components`](./src/components/) -> Focusable components folder

## Focus management and Event Forwarding

<div align="center" style="line-height:1.5;">
<p>App (Global Event Handling)
<br>↓
<br>Root (UI Container, Event Forwarder)
<br>↓
<br>Screen (Focus Manager, Focus Change Action Consumer, Event Forwarder to Focused Component)
<br>↓
<br>Component (Focusable & Event Consumer)</p>
</div>

## Key Press Event

<div align="center" style="line-height:1.5;">
<p>Global Level (App):
<br>Media Controls, Shortcut Controls, 
<br>↓
<br>Root
<br>↓
<br>Scene Level (Screen):
<br>Tab/Backtab focus change
<br>↓
<br>Interactible Level (Component):
<br>Select, Deselect, Submit, Delete, Edit
</p>
</div>
