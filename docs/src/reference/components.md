# Components and properties

This page is generated from the component catalogue; every component on the palette is listed here.

## Additional

### Badge

Class `Badge` · born as `badge1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| Caption | text | New | Appearance |
| Kind | one of: bkDefault, bkPrimary, bkInfo, bkWarning, bkDanger | bkPrimary | Appearance |
| Outline | true / false | false | Appearance |
| Count | whole number 0..99999 | 0 | Behavior |
| MaxCount | whole number 1..99999 | 99 | Behavior |
| Associate | reference to a Control | — | Behavior |
| Font | font | (default) | Appearance |
| ParentFont | true / false | true | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnClick` | `fn(&mut self)` |

### Bevel

Class `Bevel` · born as `bevel1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| Shape | one of: Box, Frame, TopLine, BottomLine, LeftLine, RightLine, Spacer | Box | Appearance |
| Style | one of: Lowered, Raised | Lowered | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnClick` | `fn(&mut self)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |

### BitBtn

Class `BitBtn` · born as `bit_btn1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| TabOrder | whole number 0..1000 | 0 | Behavior |
| TabStop | true / false | true | Behavior |
| Action | reference to a Action | — | Behavior |
| Caption | text | BitBtn | Appearance |
| Kind | one of: bkCustom, bkOK, bkCancel, bkHelp, bkClose, bkYes, bkNo | bkCustom | Behavior |
| Glyph | picture | picture | Appearance |
| NumGlyphs | whole number 1..4 | 1 | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnClick` | `fn(&mut self)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnEnter` | `fn(&mut self)` |
| `OnExit` | `fn(&mut self)` |
| `OnKeyDown` | `fn(&mut self, e: KeyEvent)` |
| `OnKeyPress` | `fn(&mut self, ch: char)` |
| `OnKeyUp` | `fn(&mut self, e: KeyEvent)` |
| `OnMouseActivate` | `fn(&mut self, e: MouseActivateEvent)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseWheel` | `fn(&mut self)` |

### Breadcrumb

Class `Breadcrumb` · born as `breadcrumb1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| TabOrder | whole number 0..1000 | 0 | Behavior |
| TabStop | true / false | true | Behavior |
| Items (read-only) | collection | 3 items | Appearance |
| Separator | text | › | Appearance |
| ItemIndex | whole number -1..100000 | -1 | Behavior |
| Style | one of: bsText, bsChevron | bsText | Appearance |
| SegmentColor | color | #45464B | Appearance |
| CurrentColor | color | #E0612B | Appearance |
| Font | font | (default) | Appearance |
| ParentFont | true / false | true | Appearance |
| Color | color | #00000000 | Appearance |
| ParentColor | true / false | true | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnChange` | `fn(&mut self)` |
| `OnClick` | `fn(&mut self)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnEnter` | `fn(&mut self)` |
| `OnExit` | `fn(&mut self)` |
| `OnKeyDown` | `fn(&mut self, e: KeyEvent)` |
| `OnKeyPress` | `fn(&mut self, ch: char)` |
| `OnKeyUp` | `fn(&mut self, e: KeyEvent)` |
| `OnMouseActivate` | `fn(&mut self, e: MouseActivateEvent)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseWheel` | `fn(&mut self)` |

### Card

Class `Card` · born as `card1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| Caption | text | Card | Appearance |
| Subtitle | text | — | Appearance |
| CornerRadius | whole number 0..100 | 8 | Appearance |
| Elevation | whole number 0..3 | 1 | Appearance |
| ShowHeader | true / false | true | Appearance |
| Font | font | (default) | Appearance |
| ParentFont | true / false | true | Appearance |
| Color | color | #00000000 | Appearance |
| ParentColor | true / false | true | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnClick` | `fn(&mut self)` |

### CategoryPanel

Class `CategoryPanel` · born as `category_panel1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| Caption | text | CategoryPanel | Appearance |
| Collapsed | true / false | false | Behavior |
| Font | font | (default) | Appearance |
| ParentFont | true / false | true | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnCollapsed` | `fn(&mut self)` |
| `OnExpanded` | `fn(&mut self)` |

### CheckListBox

Class `CheckListBox` · born as `check_list_box1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| TabOrder | whole number 0..1000 | 0 | Behavior |
| TabStop | true / false | true | Behavior |
| Items (read-only) | collection | 3 items | Appearance |
| CheckedItems | collection | (empty) | Appearance |
| BorderRadius | whole number 0..512 | 4 | Appearance |
| Font | font | (default) | Appearance |
| ParentFont | true / false | true | Appearance |
| Color | color | #00000000 | Appearance |
| ParentColor | true / false | true | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnChange` | `fn(&mut self)` |
| `OnClick` | `fn(&mut self)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnEnter` | `fn(&mut self)` |
| `OnExit` | `fn(&mut self)` |
| `OnKeyDown` | `fn(&mut self, e: KeyEvent)` |
| `OnKeyPress` | `fn(&mut self, ch: char)` |
| `OnKeyUp` | `fn(&mut self, e: KeyEvent)` |
| `OnMouseActivate` | `fn(&mut self, e: MouseActivateEvent)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseWheel` | `fn(&mut self)` |

### Chips

Class `Chips` · born as `chips1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| TabOrder | whole number 0..1000 | 0 | Behavior |
| TabStop | true / false | true | Behavior |
| Items (read-only) | collection | 3 items | Appearance |
| ItemIndex | whole number -1..100000 | -1 | Behavior |
| Removable | true / false | false | Behavior |
| BorderRadius | whole number 0..512 | 12 | Appearance |
| Font | font | (default) | Appearance |
| ParentFont | true / false | true | Appearance |
| Color | color | #00000000 | Appearance |
| ParentColor | true / false | true | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnChange` | `fn(&mut self)` |
| `OnClick` | `fn(&mut self)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnEnter` | `fn(&mut self)` |
| `OnExit` | `fn(&mut self)` |
| `OnKeyDown` | `fn(&mut self, e: KeyEvent)` |
| `OnKeyPress` | `fn(&mut self, ch: char)` |
| `OnKeyUp` | `fn(&mut self, e: KeyEvent)` |
| `OnMouseActivate` | `fn(&mut self, e: MouseActivateEvent)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseWheel` | `fn(&mut self)` |

### ColorBox

Class `ColorBox` · born as `color_box1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| TabOrder | whole number 0..1000 | 0 | Behavior |
| TabStop | true / false | true | Behavior |
| Selected | color | #000000 | Behavior |
| BorderRadius | whole number 0..512 | 4 | Appearance |
| Font | font | (default) | Appearance |
| ParentFont | true / false | true | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnChange` | `fn(&mut self)` |
| `OnClick` | `fn(&mut self)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnEnter` | `fn(&mut self)` |
| `OnExit` | `fn(&mut self)` |
| `OnKeyDown` | `fn(&mut self, e: KeyEvent)` |
| `OnKeyPress` | `fn(&mut self, ch: char)` |
| `OnKeyUp` | `fn(&mut self, e: KeyEvent)` |
| `OnMouseActivate` | `fn(&mut self, e: MouseActivateEvent)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseWheel` | `fn(&mut self)` |

### Drawer

Class `Drawer` · born as `drawer1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| Caption | text | Drawer | Appearance |
| Side | one of: dsLeft, dsRight | dsLeft | Appearance |
| Opened | true / false | false | Behavior |
| ShowHeader | true / false | true | Appearance |
| Overlay | true / false | true | Appearance |
| Font | font | (default) | Appearance |
| ParentFont | true / false | true | Appearance |
| Color | color | #00000000 | Appearance |
| ParentColor | true / false | true | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnClose` | `fn(&mut self)` |

### Grid

Class `Grid` · born as `grid1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| TabOrder | whole number 0..1000 | 0 | Behavior |
| TabStop | true / false | true | Behavior |
| ColCount | whole number 1..100 | 5 | Behavior |
| RowCount | whole number 0..100000 | 5 | Behavior |
| FixedCols | whole number 0..10 | 0 | Behavior |
| FixedRows | whole number 0..10 | 1 | Behavior |
| DefaultColWidth | whole number 20..1000 | 80 | Appearance |
| DefaultRowHeight | whole number 16..500 | 24 | Appearance |
| GridLines | true / false | true | Appearance |
| GroupBar | true / false | false | Appearance |
| Editing | true / false | false | Behavior |
| Cells | text (several lines) | — | Appearance |
| DataSource | reference to a DbRowSource | — | Data |
| TableName | text | — | Data |
| DesignRows | one of: Off, 8, 20, 50 | 8 | Data |
| BorderRadius | whole number 0..512 | 4 | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnClick` | `fn(&mut self)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnEnter` | `fn(&mut self)` |
| `OnExit` | `fn(&mut self)` |
| `OnKeyDown` | `fn(&mut self, e: KeyEvent)` |
| `OnKeyPress` | `fn(&mut self, ch: char)` |
| `OnKeyUp` | `fn(&mut self, e: KeyEvent)` |
| `OnMouseActivate` | `fn(&mut self, e: MouseActivateEvent)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseWheel` | `fn(&mut self)` |
| `OnSelectCell` | `fn(&mut self, col: usize, row: usize)` |

### Image

Class `Image` · born as `image1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| Picture | picture | picture | Appearance |
| Stretch | true / false | false | Appearance |
| Proportional | true / false | false | Appearance |
| Center | true / false | false | Appearance |
| AutoSize | true / false | false | Appearance |
| Transparent | true / false | false | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnClick` | `fn(&mut self)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |

### LinkLabel

Class `LinkLabel` · born as `link_label1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| Caption | text | LinkLabel | Appearance |
| URL | text | — | Behavior |
| Font | font | (default) | Appearance |
| ParentFont | true / false | true | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnClick` | `fn(&mut self)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |

### MaskEdit

Class `MaskEdit` · born as `mask_edit1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| TabOrder | whole number 0..1000 | 0 | Behavior |
| TabStop | true / false | true | Behavior |
| Text | text | — | Appearance |
| PasswordChar | text | — | Behavior |
| ReadOnly | true / false | false | Behavior |
| TextHint | text | — | Appearance |
| NumbersOnly | true / false | false | Behavior |
| MaxLength | whole number 0..2147483647 | 0 | Behavior |
| Alignment | one of: taLeftJustify, taCenter, taRightJustify | taLeftJustify | Appearance |
| CharCase | one of: ecNormal, ecUpperCase, ecLowerCase | ecNormal | Behavior |
| Font | font | (default) | Appearance |
| ParentFont | true / false | true | Appearance |
| Color | color | #00000000 | Appearance |
| ParentColor | true / false | true | Appearance |
| EditMask | text | — | Behavior |

#### Events

| Event | Handler signature |
|---|---|
| `OnChange` | `fn(&mut self)` |
| `OnClick` | `fn(&mut self)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnEnter` | `fn(&mut self)` |
| `OnExit` | `fn(&mut self)` |
| `OnKeyDown` | `fn(&mut self, e: KeyEvent)` |
| `OnKeyPress` | `fn(&mut self, ch: char)` |
| `OnKeyUp` | `fn(&mut self, e: KeyEvent)` |
| `OnMouseActivate` | `fn(&mut self, e: MouseActivateEvent)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseWheel` | `fn(&mut self)` |
| `OnReturn` | `fn(&mut self)` |

### MultiSelect

Class `MultiSelect` · born as `multi_select1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| TabOrder | whole number 0..1000 | 0 | Behavior |
| TabStop | true / false | true | Behavior |
| Items (read-only) | collection | 5 items | Appearance |
| SelectedItems | collection | (empty) | Appearance |
| TextHint | text | Select… | Appearance |
| DropDownCount | whole number 1..100 | 6 | Appearance |
| BorderRadius | whole number 0..512 | 4 | Appearance |
| Font | font | (default) | Appearance |
| ParentFont | true / false | true | Appearance |
| Color | color | #00000000 | Appearance |
| ParentColor | true / false | true | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnChange` | `fn(&mut self)` |
| `OnClick` | `fn(&mut self)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnEnter` | `fn(&mut self)` |
| `OnExit` | `fn(&mut self)` |
| `OnKeyDown` | `fn(&mut self, e: KeyEvent)` |
| `OnKeyPress` | `fn(&mut self, ch: char)` |
| `OnKeyUp` | `fn(&mut self, e: KeyEvent)` |
| `OnMouseActivate` | `fn(&mut self, e: MouseActivateEvent)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseWheel` | `fn(&mut self)` |

### Pagination

Class `Pagination` · born as `pagination1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| TabOrder | whole number 0..1000 | 0 | Behavior |
| TabStop | true / false | true | Behavior |
| PageCount | whole number 1..2147483647 | 10 | Behavior |
| PageIndex | whole number 0..2147483647 | 0 | Behavior |
| MaxButtons | whole number 1..100 | 7 | Appearance |
| ShowArrows | true / false | true | Appearance |
| BorderRadius | whole number 0..512 | 4 | Appearance |
| Font | font | (default) | Appearance |
| ParentFont | true / false | true | Appearance |
| Color | color | #00000000 | Appearance |
| ParentColor | true / false | true | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnChange` | `fn(&mut self)` |
| `OnClick` | `fn(&mut self)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnEnter` | `fn(&mut self)` |
| `OnExit` | `fn(&mut self)` |
| `OnKeyDown` | `fn(&mut self, e: KeyEvent)` |
| `OnKeyPress` | `fn(&mut self, ch: char)` |
| `OnKeyUp` | `fn(&mut self, e: KeyEvent)` |
| `OnMouseActivate` | `fn(&mut self, e: MouseActivateEvent)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseWheel` | `fn(&mut self)` |

### PaintBox

Class `PaintBox` · born as `paint_box1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |

#### Events

| Event | Handler signature |
|---|---|
| `OnClick` | `fn(&mut self)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |
| `OnPaint` | `fn(&mut self, canvas: &mut Canvas)` |

### ScrollBox

Class `ScrollBox` · born as `scroll_box1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| TabOrder | whole number 0..1000 | 0 | Behavior |
| AutoScroll | true / false | true | Behavior |
| HorzScrollBar | true / false | true | Appearance |
| VertScrollBar | true / false | true | Appearance |
| BorderWidth | whole number 0..20 | 1 | Appearance |
| Color | color | #00000000 | Appearance |
| ParentColor | true / false | true | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnClick` | `fn(&mut self)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnEnter` | `fn(&mut self)` |
| `OnExit` | `fn(&mut self)` |
| `OnKeyDown` | `fn(&mut self, e: KeyEvent)` |
| `OnKeyPress` | `fn(&mut self, ch: char)` |
| `OnKeyUp` | `fn(&mut self, e: KeyEvent)` |
| `OnMouseActivate` | `fn(&mut self, e: MouseActivateEvent)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseWheel` | `fn(&mut self)` |
| `OnResize` | `fn(&mut self)` |

### Shape

Class `Shape` · born as `shape1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| Shape | one of: Rectangle, Square, RoundRect, RoundSquare, Ellipse, Circle | Rectangle | Appearance |
| PenWidth | whole number 0..100 | 1 | Appearance |
| BrushColor | color | #FFFFFF | Appearance |
| PenColor | color | #000000 | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnClick` | `fn(&mut self)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |

### SpeedButton

Class `SpeedButton` · born as `speed_button1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| TabOrder | whole number 0..1000 | 0 | Behavior |
| TabStop | true / false | true | Behavior |
| Action | reference to a Action | — | Behavior |
| Caption | text | — | Appearance |
| Glyph | picture | picture | Appearance |
| NumGlyphs | whole number 1..4 | 1 | Appearance |
| Flat | true / false | true | Appearance |
| Down | true / false | false | Behavior |
| GroupIndex | whole number 0..1000 | 0 | Behavior |
| AllowAllUp | true / false | false | Behavior |

#### Events

| Event | Handler signature |
|---|---|
| `OnClick` | `fn(&mut self)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnEnter` | `fn(&mut self)` |
| `OnExit` | `fn(&mut self)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |

### SpinEdit

Class `SpinEdit` · born as `spin_edit1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| TabOrder | whole number 0..1000 | 0 | Behavior |
| TabStop | true / false | true | Behavior |
| Value | whole number -2147483648..2147483647 | 0 | Behavior |
| MinValue | whole number -2147483648..2147483647 | 0 | Behavior |
| MaxValue | whole number -2147483648..2147483647 | 0 | Behavior |
| Increment | whole number 1..2147483647 | 1 | Behavior |
| EditorEnabled | true / false | true | Behavior |
| Font | font | (default) | Appearance |
| ParentFont | true / false | true | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnChange` | `fn(&mut self)` |
| `OnClick` | `fn(&mut self)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnEnter` | `fn(&mut self)` |
| `OnExit` | `fn(&mut self)` |
| `OnKeyDown` | `fn(&mut self, e: KeyEvent)` |
| `OnKeyPress` | `fn(&mut self, ch: char)` |
| `OnKeyUp` | `fn(&mut self, e: KeyEvent)` |
| `OnMouseActivate` | `fn(&mut self, e: MouseActivateEvent)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseWheel` | `fn(&mut self)` |

### Splitter

Class `Splitter` · born as `splitter1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| MinSize | whole number 1..2000 | 30 | Behavior |
| AutoSnap | true / false | true | Behavior |
| Beveled | true / false | true | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnClick` | `fn(&mut self)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |
| `OnMoved` | `fn(&mut self)` |

### Toggle

Class `Toggle` · born as `toggle1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| TabOrder | whole number 0..1000 | 0 | Behavior |
| TabStop | true / false | true | Behavior |
| Caption | text | Toggle | Appearance |
| Checked | true / false | false | Behavior |
| Font | font | (default) | Appearance |
| ParentFont | true / false | true | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnChange` | `fn(&mut self)` |
| `OnClick` | `fn(&mut self)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnEnter` | `fn(&mut self)` |
| `OnExit` | `fn(&mut self)` |
| `OnKeyDown` | `fn(&mut self, e: KeyEvent)` |
| `OnKeyPress` | `fn(&mut self, ch: char)` |
| `OnKeyUp` | `fn(&mut self, e: KeyEvent)` |
| `OnMouseActivate` | `fn(&mut self, e: MouseActivateEvent)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseWheel` | `fn(&mut self)` |

### TreeView

Class `TreeView` · born as `tree_view1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| TabOrder | whole number 0..1000 | 0 | Behavior |
| TabStop | true / false | true | Behavior |
| Items (read-only) | collection | 4 items | Appearance |
| Images | reference to a ImageList | — | Appearance |
| BorderRadius | whole number 0..512 | 4 | Appearance |
| Font | font | (default) | Appearance |
| ParentFont | true / false | true | Appearance |
| Color | color | #00000000 | Appearance |
| ParentColor | true / false | true | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnChange` | `fn(&mut self)` |
| `OnChanging` | `fn(&mut self, allow_change: &mut bool)` |
| `OnClick` | `fn(&mut self)` |
| `OnCollapsed` | `fn(&mut self)` |
| `OnCollapsing` | `fn(&mut self)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnEnter` | `fn(&mut self)` |
| `OnExit` | `fn(&mut self)` |
| `OnExpanded` | `fn(&mut self)` |
| `OnExpanding` | `fn(&mut self)` |
| `OnKeyDown` | `fn(&mut self, e: KeyEvent)` |
| `OnKeyPress` | `fn(&mut self, ch: char)` |
| `OnKeyUp` | `fn(&mut self, e: KeyEvent)` |
| `OnMouseActivate` | `fn(&mut self, e: MouseActivateEvent)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseWheel` | `fn(&mut self)` |

## Dialogs

### MessageDialog

Class `MessageDialog` · born as `message_dialog1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 28 × 28 | Layout |
| Title | text | — | Appearance |
| Text | text | — | Appearance |
| Kind | one of: mtInformation, mtWarning, mtError, mtConfirmation | mtInformation | Appearance |
| Buttons | one of: mbOK, mbOKCancel, mbYesNo, mbYesNoCancel | mbOK | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnClose` | `fn(&mut self)` |

### OpenDialog

Class `OpenDialog` · born as `open_dialog1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 28 × 28 | Layout |
| Title | text | Open | Appearance |
| Filter | text | All files (*.*)\|*.* | Behavior |
| FileName | text | — | Behavior |
| InitialDir | text | — | Behavior |
| DefaultExt | text | — | Behavior |

### SaveDialog

Class `SaveDialog` · born as `save_dialog1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 28 × 28 | Layout |
| Title | text | Save As | Appearance |
| Filter | text | All files (*.*)\|*.* | Behavior |
| FileName | text | — | Behavior |
| InitialDir | text | — | Behavior |
| DefaultExt | text | — | Behavior |

## Standard

### ActionList

Class `ActionList` · born as `action_list1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 28 × 28 | Layout |
| Enabled | true / false | true | Behavior |
| Images | reference to a ImageList | — | Appearance |

### Button

Class `Button` · born as `button1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| TabOrder | whole number 0..1000 | 0 | Behavior |
| TabStop | true / false | true | Behavior |
| Action | reference to a Action | — | Behavior |
| Caption | text | Button | Appearance |
| Font | font | (default) | Appearance |
| ParentFont | true / false | true | Appearance |
| Color | color | #00000000 | Appearance |
| ParentColor | true / false | true | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnClick` | `fn(&mut self)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnEnter` | `fn(&mut self)` |
| `OnExit` | `fn(&mut self)` |
| `OnKeyDown` | `fn(&mut self, e: KeyEvent)` |
| `OnKeyPress` | `fn(&mut self, ch: char)` |
| `OnKeyUp` | `fn(&mut self, e: KeyEvent)` |
| `OnMouseActivate` | `fn(&mut self, e: MouseActivateEvent)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseWheel` | `fn(&mut self)` |

### CheckBox

Class `CheckBox` · born as `check_box1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| TabOrder | whole number 0..1000 | 0 | Behavior |
| TabStop | true / false | true | Behavior |
| Caption | text | CheckBox | Appearance |
| Checked | true / false | false | Behavior |
| State | one of: cbUnchecked, cbChecked, cbGrayed | cbUnchecked | Behavior |
| AllowGrayed | true / false | false | Behavior |
| Alignment | one of: taLeftJustify, taRightJustify | taRightJustify | Appearance |
| Font | font | (default) | Appearance |
| ParentFont | true / false | true | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnChange` | `fn(&mut self)` |
| `OnClick` | `fn(&mut self)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnEnter` | `fn(&mut self)` |
| `OnExit` | `fn(&mut self)` |
| `OnKeyDown` | `fn(&mut self, e: KeyEvent)` |
| `OnKeyPress` | `fn(&mut self, ch: char)` |
| `OnKeyUp` | `fn(&mut self, e: KeyEvent)` |
| `OnMouseActivate` | `fn(&mut self, e: MouseActivateEvent)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseWheel` | `fn(&mut self)` |

### ComboBox

Class `ComboBox` · born as `combo_box1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| TabOrder | whole number 0..1000 | 0 | Behavior |
| TabStop | true / false | true | Behavior |
| Items (read-only) | collection | 3 items | Appearance |
| ItemIndex | whole number -1..100000 | -1 | Behavior |
| Text | text | — | Appearance |
| Style | one of: DropDownList, DropDown | DropDownList | Appearance |
| Sorted | true / false | false | Behavior |
| DropDownCount | whole number 1..100 | 8 | Appearance |
| Font | font | (default) | Appearance |
| ParentFont | true / false | true | Appearance |
| Color | color | #00000000 | Appearance |
| ParentColor | true / false | true | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnChange` | `fn(&mut self)` |
| `OnClick` | `fn(&mut self)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnEnter` | `fn(&mut self)` |
| `OnExit` | `fn(&mut self)` |
| `OnKeyDown` | `fn(&mut self, e: KeyEvent)` |
| `OnKeyPress` | `fn(&mut self, ch: char)` |
| `OnKeyUp` | `fn(&mut self, e: KeyEvent)` |
| `OnMouseActivate` | `fn(&mut self, e: MouseActivateEvent)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseWheel` | `fn(&mut self)` |
| `OnSelect` | `fn(&mut self)` |

### Edit

Class `Edit` · born as `edit1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| TabOrder | whole number 0..1000 | 0 | Behavior |
| TabStop | true / false | true | Behavior |
| Text | text | — | Appearance |
| PasswordChar | text | — | Behavior |
| ReadOnly | true / false | false | Behavior |
| TextHint | text | — | Appearance |
| NumbersOnly | true / false | false | Behavior |
| MaxLength | whole number 0..2147483647 | 0 | Behavior |
| Alignment | one of: taLeftJustify, taCenter, taRightJustify | taLeftJustify | Appearance |
| CharCase | one of: ecNormal, ecUpperCase, ecLowerCase | ecNormal | Behavior |
| Font | font | (default) | Appearance |
| ParentFont | true / false | true | Appearance |
| Color | color | #00000000 | Appearance |
| ParentColor | true / false | true | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnChange` | `fn(&mut self)` |
| `OnClick` | `fn(&mut self)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnEnter` | `fn(&mut self)` |
| `OnExit` | `fn(&mut self)` |
| `OnKeyDown` | `fn(&mut self, e: KeyEvent)` |
| `OnKeyPress` | `fn(&mut self, ch: char)` |
| `OnKeyUp` | `fn(&mut self, e: KeyEvent)` |
| `OnMouseActivate` | `fn(&mut self, e: MouseActivateEvent)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseWheel` | `fn(&mut self)` |
| `OnReturn` | `fn(&mut self)` |

### GroupBox

Class `GroupBox` · born as `group_box1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| TabOrder | whole number 0..1000 | 0 | Behavior |
| Caption | text | GroupBox | Appearance |
| Font | font | (default) | Appearance |
| ParentFont | true / false | true | Appearance |
| Color | color | #00000000 | Appearance |
| ParentColor | true / false | true | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnClick` | `fn(&mut self)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnEnter` | `fn(&mut self)` |
| `OnExit` | `fn(&mut self)` |
| `OnKeyDown` | `fn(&mut self, e: KeyEvent)` |
| `OnKeyPress` | `fn(&mut self, ch: char)` |
| `OnKeyUp` | `fn(&mut self, e: KeyEvent)` |
| `OnMouseActivate` | `fn(&mut self, e: MouseActivateEvent)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseWheel` | `fn(&mut self)` |

### ImageList

Class `ImageList` · born as `image_list1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 28 × 28 | Layout |
| Width | whole number 8..256 | 16 | Appearance |
| Height | whole number 8..256 | 16 | Appearance |
| Images (read-only) | collection | (empty) | Advanced |

### Label

Class `Label` · born as `label1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| Caption | text | Label | Appearance |
| Alignment | one of: taLeftJustify, taCenter, taRightJustify | taLeftJustify | Appearance |
| Layout | one of: tlTop, tlCenter, tlBottom | tlTop | Appearance |
| WordWrap | true / false | false | Appearance |
| AutoSize | true / false | false | Appearance |
| FocusControl | reference to a WinControl | — | Behavior |
| Font | font | (default) | Appearance |
| ParentFont | true / false | true | Appearance |
| Color | color | #00000000 | Appearance |
| ParentColor | true / false | true | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnClick` | `fn(&mut self)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |

### ListBox

Class `ListBox` · born as `list_box1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| TabOrder | whole number 0..1000 | 0 | Behavior |
| TabStop | true / false | true | Behavior |
| Items (read-only) | collection | 3 items | Appearance |
| ItemIndex | whole number -1..100000 | -1 | Behavior |
| MultiSelect | true / false | false | Behavior |
| Sorted | true / false | false | Behavior |
| Font | font | (default) | Appearance |
| ParentFont | true / false | true | Appearance |
| Color | color | #00000000 | Appearance |
| ParentColor | true / false | true | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnChange` | `fn(&mut self)` |
| `OnClick` | `fn(&mut self)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnEnter` | `fn(&mut self)` |
| `OnExit` | `fn(&mut self)` |
| `OnKeyDown` | `fn(&mut self, e: KeyEvent)` |
| `OnKeyPress` | `fn(&mut self, ch: char)` |
| `OnKeyUp` | `fn(&mut self, e: KeyEvent)` |
| `OnMouseActivate` | `fn(&mut self, e: MouseActivateEvent)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseWheel` | `fn(&mut self)` |

### MainMenu

Class `MainMenu` · born as `main_menu1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 28 × 28 | Layout |
| Images | reference to a ImageList | — | Appearance |

### Memo

Class `Memo` · born as `memo1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| TabOrder | whole number 0..1000 | 0 | Behavior |
| TabStop | true / false | true | Behavior |
| Lines | text (several lines) | Line1
Line2
Line3 | Appearance |
| ReadOnly | true / false | false | Behavior |
| WordWrap | true / false | true | Appearance |
| WantReturns | true / false | true | Behavior |
| WantTabs | true / false | false | Behavior |
| MaxLength | whole number 0..2147483647 | 0 | Behavior |
| ScrollBars | one of: ssNone, ssHorizontal, ssVertical, ssBoth | ssNone | Appearance |
| Font | font | (default) | Appearance |
| ParentFont | true / false | true | Appearance |
| Color | color | #00000000 | Appearance |
| ParentColor | true / false | true | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnChange` | `fn(&mut self)` |
| `OnClick` | `fn(&mut self)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnEnter` | `fn(&mut self)` |
| `OnExit` | `fn(&mut self)` |
| `OnKeyDown` | `fn(&mut self, e: KeyEvent)` |
| `OnKeyPress` | `fn(&mut self, ch: char)` |
| `OnKeyUp` | `fn(&mut self, e: KeyEvent)` |
| `OnMouseActivate` | `fn(&mut self, e: MouseActivateEvent)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseWheel` | `fn(&mut self)` |

### PageControl

Class `PageControl` · born as `page_control1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| TabOrder | whole number 0..1000 | 0 | Behavior |
| TabStop | true / false | true | Behavior |
| ActivePageIndex | whole number 0..100 | 0 | Behavior |
| TabPosition | one of: tpTop, tpBottom | tpTop | Appearance |
| Font | font | (default) | Appearance |
| ParentFont | true / false | true | Appearance |
| Color | color | #00000000 | Appearance |
| ParentColor | true / false | true | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnChange` | `fn(&mut self)` |
| `OnChanging` | `fn(&mut self, allow_change: &mut bool)` |
| `OnClick` | `fn(&mut self)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnEnter` | `fn(&mut self)` |
| `OnExit` | `fn(&mut self)` |
| `OnKeyDown` | `fn(&mut self, e: KeyEvent)` |
| `OnKeyPress` | `fn(&mut self, ch: char)` |
| `OnKeyUp` | `fn(&mut self, e: KeyEvent)` |
| `OnMouseActivate` | `fn(&mut self, e: MouseActivateEvent)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseWheel` | `fn(&mut self)` |
| `OnResize` | `fn(&mut self)` |

### Panel

Class `Panel` · born as `panel1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| TabOrder | whole number 0..1000 | 0 | Behavior |
| Caption | text | — | Appearance |
| BorderWidth | whole number 0..20 | 1 | Appearance |
| Font | font | (default) | Appearance |
| ParentFont | true / false | true | Appearance |
| Color | color | #00000000 | Appearance |
| ParentColor | true / false | true | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnClick` | `fn(&mut self)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnEnter` | `fn(&mut self)` |
| `OnExit` | `fn(&mut self)` |
| `OnKeyDown` | `fn(&mut self, e: KeyEvent)` |
| `OnKeyPress` | `fn(&mut self, ch: char)` |
| `OnKeyUp` | `fn(&mut self, e: KeyEvent)` |
| `OnMouseActivate` | `fn(&mut self, e: MouseActivateEvent)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseWheel` | `fn(&mut self)` |
| `OnResize` | `fn(&mut self)` |

### PopupMenu

Class `PopupMenu` · born as `popup_menu1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 28 × 28 | Layout |
| Images | reference to a ImageList | — | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnPopup` | `fn(&mut self)` |

### RadioButton

Class `RadioButton` · born as `radio_button1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| TabOrder | whole number 0..1000 | 0 | Behavior |
| TabStop | true / false | true | Behavior |
| Caption | text | RadioButton | Appearance |
| Checked | true / false | false | Behavior |
| BorderRadius | whole number 0..512 | 8 | Appearance |
| Font | font | (default) | Appearance |
| ParentFont | true / false | true | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnChange` | `fn(&mut self)` |
| `OnClick` | `fn(&mut self)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnEnter` | `fn(&mut self)` |
| `OnExit` | `fn(&mut self)` |
| `OnKeyDown` | `fn(&mut self, e: KeyEvent)` |
| `OnKeyPress` | `fn(&mut self, ch: char)` |
| `OnKeyUp` | `fn(&mut self, e: KeyEvent)` |
| `OnMouseActivate` | `fn(&mut self, e: MouseActivateEvent)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseWheel` | `fn(&mut self)` |

### RadioGroup

Class `RadioGroup` · born as `radio_group1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| TabOrder | whole number 0..1000 | 0 | Behavior |
| TabStop | true / false | true | Behavior |
| Caption | text | RadioGroup | Appearance |
| Items (read-only) | collection | 3 items | Appearance |
| ItemIndex | whole number -1..100000 | -1 | Behavior |
| Columns | whole number 1..16 | 1 | Appearance |
| Font | font | (default) | Appearance |
| ParentFont | true / false | true | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnChange` | `fn(&mut self)` |
| `OnClick` | `fn(&mut self)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnEnter` | `fn(&mut self)` |
| `OnExit` | `fn(&mut self)` |
| `OnKeyDown` | `fn(&mut self, e: KeyEvent)` |
| `OnKeyPress` | `fn(&mut self, ch: char)` |
| `OnKeyUp` | `fn(&mut self, e: KeyEvent)` |
| `OnMouseActivate` | `fn(&mut self, e: MouseActivateEvent)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseWheel` | `fn(&mut self)` |

### ScrollBar

Class `ScrollBar` · born as `scroll_bar1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| TabOrder | whole number 0..1000 | 0 | Behavior |
| TabStop | true / false | true | Behavior |
| Kind | one of: sbHorizontal, sbVertical | sbHorizontal | Appearance |
| Min | whole number -1000000..1000000 | 0 | Behavior |
| Max | whole number -1000000..1000000 | 100 | Behavior |
| Position | whole number 0..1000000 | 0 | Behavior |
| SmallChange | whole number 1..10000 | 1 | Behavior |
| LargeChange | whole number 1..10000 | 10 | Behavior |

#### Events

| Event | Handler signature |
|---|---|
| `OnChange` | `fn(&mut self)` |
| `OnClick` | `fn(&mut self)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnEnter` | `fn(&mut self)` |
| `OnExit` | `fn(&mut self)` |
| `OnKeyDown` | `fn(&mut self, e: KeyEvent)` |
| `OnKeyPress` | `fn(&mut self, ch: char)` |
| `OnKeyUp` | `fn(&mut self, e: KeyEvent)` |
| `OnMouseActivate` | `fn(&mut self, e: MouseActivateEvent)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseWheel` | `fn(&mut self)` |
| `OnScroll` | `fn(&mut self)` |

## System

### Timer

Class `Timer` · born as `timer1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 28 × 28 | Layout |
| Enabled | true / false | true | Behavior |
| Interval | whole number 1..3600000 | 1000 | Behavior |

#### Events

| Event | Handler signature |
|---|---|
| `OnTimer` | `fn(&mut self)` |

## Win32

### DateTimePicker

Class `DateTimePicker` · born as `date_time_picker1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| TabOrder | whole number 0..1000 | 0 | Behavior |
| TabStop | true / false | true | Behavior |
| Kind | one of: dtkDate, dtkTime | dtkDate | Behavior |
| DateTime | text | 2000-01-01 | Behavior |
| MinDate | text | — | Behavior |
| MaxDate | text | — | Behavior |
| BorderRadius | whole number 0..512 | 4 | Appearance |
| Font | font | (default) | Appearance |
| ParentFont | true / false | true | Appearance |
| Color | color | #00000000 | Appearance |
| ParentColor | true / false | true | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnChange` | `fn(&mut self)` |
| `OnClick` | `fn(&mut self)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnEnter` | `fn(&mut self)` |
| `OnExit` | `fn(&mut self)` |
| `OnKeyDown` | `fn(&mut self, e: KeyEvent)` |
| `OnKeyPress` | `fn(&mut self, ch: char)` |
| `OnKeyUp` | `fn(&mut self, e: KeyEvent)` |
| `OnMouseActivate` | `fn(&mut self, e: MouseActivateEvent)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseWheel` | `fn(&mut self)` |

### ListView

Class `ListView` · born as `list_view1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| TabOrder | whole number 0..1000 | 0 | Behavior |
| TabStop | true / false | true | Behavior |
| Items (read-only) | collection | 4 items | Appearance |
| ItemIndex | whole number -1..100000 | -1 | Behavior |
| RowSelect | true / false | true | Behavior |
| GridLines | true / false | false | Appearance |
| ShowColumnHeaders | true / false | true | Appearance |
| BorderRadius | whole number 0..512 | 4 | Appearance |
| Font | font | (default) | Appearance |
| ParentFont | true / false | true | Appearance |
| Color | color | #00000000 | Appearance |
| ParentColor | true / false | true | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnChange` | `fn(&mut self)` |
| `OnClick` | `fn(&mut self)` |
| `OnColumnClick` | `fn(&mut self, column: usize)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnEnter` | `fn(&mut self)` |
| `OnExit` | `fn(&mut self)` |
| `OnKeyDown` | `fn(&mut self, e: KeyEvent)` |
| `OnKeyPress` | `fn(&mut self, ch: char)` |
| `OnKeyUp` | `fn(&mut self, e: KeyEvent)` |
| `OnMouseActivate` | `fn(&mut self, e: MouseActivateEvent)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseWheel` | `fn(&mut self)` |
| `OnSelectItem` | `fn(&mut self, index: usize, selected: bool)` |

### ProgressBar

Class `ProgressBar` · born as `progress_bar1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| TabOrder | whole number 0..1000 | 0 | Behavior |
| TabStop | true / false | true | Behavior |
| Min | whole number -1000000..1000000 | 0 | Behavior |
| Max | whole number -1000000..1000000 | 100 | Behavior |
| Position | whole number 0..1000000 | 0 | Behavior |
| Step | whole number 1..10000 | 10 | Behavior |
| Orientation | one of: Horizontal, Vertical | Horizontal | Appearance |
| Smooth | true / false | true | Appearance |
| BorderRadius | whole number 0..512 | 3 | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnClick` | `fn(&mut self)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnEnter` | `fn(&mut self)` |
| `OnExit` | `fn(&mut self)` |
| `OnKeyDown` | `fn(&mut self, e: KeyEvent)` |
| `OnKeyPress` | `fn(&mut self, ch: char)` |
| `OnKeyUp` | `fn(&mut self, e: KeyEvent)` |
| `OnMouseActivate` | `fn(&mut self, e: MouseActivateEvent)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseWheel` | `fn(&mut self)` |

### StatusBar

Class `StatusBar` · born as `status_bar1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| TabOrder | whole number 0..1000 | 0 | Behavior |
| TabStop | true / false | true | Behavior |
| SimpleText | text | — | Appearance |
| SimplePanel | true / false | true | Behavior |
| Font | font | (default) | Appearance |
| ParentFont | true / false | true | Appearance |
| Color | color | #00000000 | Appearance |
| ParentColor | true / false | true | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnClick` | `fn(&mut self)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnEnter` | `fn(&mut self)` |
| `OnExit` | `fn(&mut self)` |
| `OnKeyDown` | `fn(&mut self, e: KeyEvent)` |
| `OnKeyPress` | `fn(&mut self, ch: char)` |
| `OnKeyUp` | `fn(&mut self, e: KeyEvent)` |
| `OnMouseActivate` | `fn(&mut self, e: MouseActivateEvent)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseWheel` | `fn(&mut self)` |

### ToolBar

Class `ToolBar` · born as `tool_bar1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| TabOrder | whole number 0..1000 | 0 | Behavior |
| Flat | true / false | true | Appearance |
| ShowCaptions | true / false | false | Appearance |
| ButtonWidth | whole number 16..256 | 24 | Appearance |
| ButtonHeight | whole number 16..256 | 24 | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnClick` | `fn(&mut self)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnEnter` | `fn(&mut self)` |
| `OnExit` | `fn(&mut self)` |
| `OnKeyDown` | `fn(&mut self, e: KeyEvent)` |
| `OnKeyPress` | `fn(&mut self, ch: char)` |
| `OnKeyUp` | `fn(&mut self, e: KeyEvent)` |
| `OnMouseActivate` | `fn(&mut self, e: MouseActivateEvent)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseWheel` | `fn(&mut self)` |

### TrackBar

Class `TrackBar` · born as `track_bar1` · targets: all

#### Properties

| Property | Kind | Default | Category |
|---|---|---|---|
| Name | text | — | Identity |
| Rect | position and size | 0, 0, 80 × 24 | Layout |
| Enabled | true / false | true | Behavior |
| Visible | true / false | true | Behavior |
| Hint | text | — | Behavior |
| ShowHint | true / false | true | Behavior |
| Cursor | mouse cursor | default | Appearance |
| PopupMenu | reference to a PopupMenu | — | Behavior |
| Anchors | any of: left, top, right, bottom | [left,top] | Layout |
| Align | one of: None, Top, Bottom, Left, Right, Client | None | Layout |
| Margins | edges (left, top, right, bottom) | 3, 3, 3, 3 | Layout |
| AlignWithMargins | true / false | false | Layout |
| TabOrder | whole number 0..1000 | 0 | Behavior |
| TabStop | true / false | true | Behavior |
| Min | whole number -1000000..1000000 | 0 | Behavior |
| Max | whole number -1000000..1000000 | 10 | Behavior |
| Position | whole number 0..1000000 | 0 | Behavior |
| Orientation | one of: Horizontal, Vertical | Horizontal | Appearance |
| PageSize | whole number 1..1000 | 2 | Behavior |
| Look | one of: Minimal, Box, Round | Minimal | Appearance |

#### Events

| Event | Handler signature |
|---|---|
| `OnChange` | `fn(&mut self)` |
| `OnClick` | `fn(&mut self)` |
| `OnDblClick` | `fn(&mut self)` |
| `OnEnter` | `fn(&mut self)` |
| `OnExit` | `fn(&mut self)` |
| `OnKeyDown` | `fn(&mut self, e: KeyEvent)` |
| `OnKeyPress` | `fn(&mut self, ch: char)` |
| `OnKeyUp` | `fn(&mut self, e: KeyEvent)` |
| `OnMouseActivate` | `fn(&mut self, e: MouseActivateEvent)` |
| `OnMouseDown` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseEnter` | `fn(&mut self)` |
| `OnMouseLeave` | `fn(&mut self)` |
| `OnMouseMove` | `fn(&mut self, e: MouseMoveEvent)` |
| `OnMouseUp` | `fn(&mut self, e: MouseEvent)` |
| `OnMouseWheel` | `fn(&mut self)` |
