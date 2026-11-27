use makepad_widgets::*;

live_design! {
    link crawrad_panels;

    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;
    use link::crawrad_theme::*;

    // ----------------------------------------------------------------- AI Cards
    // Each assistant card is an autonomous, decoupled, and self-contained component.

    // The live status line: a 2 px bar on the left (the row's own background),
    // beside the italic line and its monospace sub. Green while the agent works.
    pub CrawradAiStatusRow = <View> {
        width: Fill, height: Fit, flow: Down, spacing: 2.0, padding: { left: 8.0 }, show_bg: true,
        draw_bg: {
            color: (CRAWRAD_CHIP_ACP)
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.rect(0.0, 0.0, 2.0, self.rect_size.y);
                sdf.fill(self.color);
                return sdf.result;
            }
        }
        st_line = <CrawradLabel> { width: Fill, text: "", draw_text: { text_style: <THEME_FONT_ITALIC> { font_size: (CRAWRAD_FONTE_CORPO) }, color: (CRAWRAD_TEXTO_PRIMARIO) } }
        st_sub = <CrawradLabel> { width: Fill, text: "", draw_text: { text_style: <THEME_FONT_CODE> { font_size: (CRAWRAD_FONTE_CORPO) }, color: (CRAWRAD_TEXTO_TERCIARIO) } }
    }

    // One shortcut of the hero grid.
    pub CrawradAiShortcut = <CrawradButton> {
        width: Fill, height: 54.0, align: { x: 0.0, y: 0.5 }, padding: { left: 16.0 },
        draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 11.0 } }
    }
    // One row of two shortcuts.
    AiShortcutRow = <View> { width: Fill, height: Fit, flow: Right, spacing: 10.0 }

    pub CrawradAiHeroView = <View> {
        width: Fill, height: Fit,
        flow: Down,
        spacing: 24.0,

        session_row = <View> {
            width: Fill, height: Fit, flow: Right, align: { y: 0.5 },
            lbl_session_status = <CrawradLabel> { draw_text: { color: (CRAWRAD_TEXTO_PRIMARIO) }, text: "🟢 New session" }
            session_spacer = <CrawradFiller> {}
            lbl_session_count = <CrawradLabelMuted> { text: "0 messages" }
        }

        hero_block = <View> {
            width: Fill, height: Fit, flow: Down, align: { x: 0.5 },
            margin: { top: 28.0, bottom: 20.0 },
            spacing: 8.0,

            lbl_hero_badge = <CrawradLabel> { align: { x: 0.5 }, draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 22.0 }, color: (CRAWRAD_ACENTO_TEXTO) }, text: "⚡ Aefos AI" }
            lbl_hero_title = <CrawradLabel> { align: { x: 0.5 }, draw_text: { text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_CABECALHO) }, color: (CRAWRAD_TEXTO_PRIMARIO) }, text: "What are we building today?" }
            lbl_hero_subtitle = <CrawradLabelMuted> { align: { x: 0.5 }, text: "Aefos is ready. Pick a shortcut or just start typing." }
        }

        shortcuts_grid = <View> {
            width: Fill, height: Fit, flow: Down, spacing: 12.0,

            row_1 = <AiShortcutRow> {
                btn_card_explain = <CrawradAiShortcut> { text: "❓  Explain" }
                btn_card_refactor = <CrawradAiShortcut> { text: "⇄  Refactor" }
            }
            row_2 = <AiShortcutRow> {
                btn_card_test = <CrawradAiShortcut> { text: "🧪  Test" }
                btn_card_document = <CrawradAiShortcut> { text: "📄  Document" }
            }
            row_3 = <AiShortcutRow> {
                btn_card_find = <CrawradAiShortcut> { text: "🔍  Find" }
                btn_card_optimize = <CrawradAiShortcut> { text: "⚡  Optimize" }
            }
            row_teach = <View> {
                width: Fill, height: Fit, align: { x: 0.5 }, margin: { top: 18.0 },
                teach_card = <View> { width: 250.0, height: 68.0, flow: Overlay,
                    btn_card_teach = <CrawradAiShortcut> { width: Fill, height: Fill, text: "",
                        draw_bg: { color: (CRAWRAD_TEACH_FILL), color_hover: (CRAWRAD_TEACH_HOVER), border_color_1: (CRAWRAD_ACENTO), border_color_2: (CRAWRAD_ACENTO) }
                    }
                    teach_content = <View> {
                        width: Fill, height: Fill, flow: Right, align: { x: 0.5, y: 0.5 }, spacing: 10.0,
                        teach_crab = <Image> { width: 34.0, height: 29.0, fit: Stretch }
                        <View> {
                            width: Fit, height: Fit, flow: Down, spacing: 1.0,
                            <CrawradLabel> { text: "Rust Teacher", draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 10.0 }, color: (CRAWRAD_TEXTO_PRIMARIO) } }
                            <CrawradLabelMuted> { text: "Learn why, not just how" }
                        }
                    }
                }
            }
        }

        ready_row = <View> {
            width: Fill, height: Fit,
            align: { x: 0.5 },
            margin: { top: 32.0, bottom: 16.0 },
            lbl_ready_status = <CrawradLabelMuted> {
                align: { x: 0.5 },
                text: "⚪ Checking the AI agent…"
            }
        }
    }

    pub CrawradAiCopilotCard = <CrawradCard> {
        visible: false,
        sender_row = <View> {
            width: Fill, height: Fit,
            flow: Right, align: { y: 0.5 },
            sender_name = <CrawradLabel> {
                draw_text: { color: (CRAWRAD_ACENTO_TEXTO) },
                text: "Copilot"
            }
        }

        msg_text = <CrawradLabel> {
            width: Fill,
            margin: { top: (CRAWRAD_ESPACO_M) },
            draw_text: { color: (CRAWRAD_TEXTO_SECUNDARIO) },
            text: "Aefos AI assistant ready. Ask questions, refactor code, or plan tasks."
        }
    }

    pub CrawradAiCodeBlock = <CrawradOutlined> {
        visible: false,
        width: Fill, height: Fit,
        flow: Down,
        margin: { top: 4.0, bottom: 4.0 },
        draw_bg: {
            uniform border_radius: 4.0
            color: (CRAWRAD_GROUND)
        }

        code_header = <View> {
            width: Fill, height: 24.0,
            flow: Right, align: { y: 0.5 },
            padding: { left: 8.0, right: 8.0 },
            show_bg: true,
            draw_bg: { color: (CRAWRAD_FLUTUANTE) }
            lbl_code_lang = <CrawradLabelMuted> {
                draw_text: { text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_PEQUENO) } }
                text: "code"
            }
            code_spacer = <CrawradFiller> {}
            btn_copy_code = <CrawradButton> {
                height: 18.0,
                padding: { left: 6.0, right: 6.0 },
                draw_text: { text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_PEQUENO) } },
                text: "Copy code"
            }
        }

        code_scroll = <ScrollYView> {
            width: Fill, height: Fit,
            code_text = <CrawradLabel> {
                width: Fill,
                padding: { left: 8.0, right: 8.0, top: 6.0, bottom: 6.0 },
                draw_text: {
                    text_style: <THEME_FONT_CODE> { font_size: (CRAWRAD_FONTE_MONO) },
                    color: (CRAWRAD_TEXTO_PRIMARIO)
                },
                text: ""
            }
        }
    }

    pub CrawradAiToolCard = <CrawradOutlined> {
        visible: false,
        width: Fill, height: Fit,
        flow: Down,
        margin: { top: 4.0, bottom: 4.0 },
        padding: { left: 8.0, right: 8.0, top: 6.0, bottom: 6.0 },
        spacing: 4.0,
        draw_bg: {
            uniform border_radius: 4.0
            color: (CRAWRAD_FLUTUANTE)
        }

        tool_header = <View> {
            width: Fill, height: Fit,
            flow: Right, align: { y: 0.5 },
            spacing: 6.0,

            lbl_tool_icon = <CrawradLabel> {
                draw_text: { text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_CORPO) }, color: (CRAWRAD_ACENTO_TEXTO) },
                text: "🔧"
            }
            lbl_tool_name = <CrawradLabel> {
                draw_text: { text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_PEQUENO) }, color: (CRAWRAD_TEXTO_PRIMARIO) },
                text: "tool"
            }
            tool_spacer = <CrawradFiller> {}
            badge_tool_status = <View> {
                width: Fit, height: 18.0,
                padding: { left: 6.0, right: 6.0 },
                align: { y: 0.5 },
                show_bg: true,
                draw_bg: {
                    uniform border_radius: 3.0
                    color: (CRAWRAD_SELECAO)
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, self.border_radius);
                        sdf.fill(self.color);
                        return sdf.result;
                    }
                }
                lbl_tool_status = <CrawradLabel> {
                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_PEQUENO) }, color: (CRAWRAD_TEXTO_PRIMARIO) },
                    text: "Running"
                }
            }
        }
        lbl_tool_args = <CrawradLabelMuted> {
            width: Fill,
            draw_text: { text_style: <THEME_FONT_CODE> { font_size: (CRAWRAD_FONTE_MONO) } },
            text: ""
        }
    }

    // The agent chip's dot; its colour says how the last answer came.
    pub CrawradMenuSlot = <CrawradButton> { align: { x: 0.0, y: 0.5 }, width: Fill, height: 24.0, visible: false }

    pub CrawradChipDot = <View> {
        visible: false, width: 7.0, height: 7.0, show_bg: true,
        draw_bg: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.circle(3.5, 3.5, 3.5);
                sdf.fill(self.color);
                return sdf.result;
            }
        }
    }

    pub CrawradAiUserCard = <View> {
        visible: false,
        width: Fill, height: Fit,
        flow: Right, align: { y: 1.0 },
        spacing: 8.0,

        u_bubble = <CrawradCardUser> {
            width: Fill, height: Fit,
            margin: { left: 0.0, right: 0.0, top: 4.0, bottom: 4.0 },

            u_sender_row = <View> {
                visible: false,
                width: Fill, height: Fit,
                flow: Right, align: { y: 0.5 },
                u_sender_name = <CrawradLabel> {
                    draw_text: { color: (CRAWRAD_TEXTO_TERCIARIO) },
                    text: "Developer"
                }
            }
            u_row = <View> {
                width: Fill, height: Fit,
                flow: Right, align: { y: 1.0 },
                spacing: 10.0,

                u_msg_text = <CrawradSelectableText> {
                    width: Fit,
                    margin: { top: 0.0 },
                    draw_text: {
                        color: (CRAWRAD_BRANCO),
                        text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_CORPO) }
                    },
                    text: ""
                }
                u_meta_row = <View> {
                    width: Fit, height: Fit,
                    flow: Right, align: { y: 1.0 },
                    spacing: 4.0,

                    u_meta = <CrawradLabel> {
                        width: Fit,
                        draw_text: {
                            color: (CRAWRAD_BUBBLE_META),
                            text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_PEQUENO) }
                        },
                        text: ""
                    }
                    u_mark = <CrawradLabel> {
                        width: Fit,
                        draw_text: {
                            color: (CRAWRAD_BUBBLE_META),
                            text_style: <CRAWRAD_FONT_ICONS> { font_size: (CRAWRAD_FONTE_ICONE_LINHA) }
                        },
                        text: ""
                    }
                }
            }
        }
        u_avatar = <View> {
            width: 28.0, height: 28.0,
            margin: { bottom: 4.0 },
            align: { x: 0.5, y: 0.5 },
            show_bg: true,
            draw_bg: {
                color: (CRAWRAD_AVATAR_USER)
                uniform glyph_color: (CRAWRAD_BRANCO)
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.circle(14.0, 14.0, 14.0);
                    sdf.fill(self.color);
                    sdf.circle(14.0, 9.5, 3.8);
                    sdf.fill(self.glyph_color);
                    sdf.circle(14.0, 25.5, 8.5);
                    sdf.fill(self.glyph_color);
                    let mask = clamp(14.0 - length(self.pos * self.rect_size - vec2(14.0, 14.0)) + 0.5, 0.0, 1.0);
                    return vec4(sdf.result.rgb, sdf.result.a * mask);
                }
            }
        }
    }

    pub CrawradAiReplyCard = <View> {
        visible: false,
        width: Fill, height: Fit,
        flow: Right, align: { y: 0.0 },
        spacing: 8.0,

        r_avatar = <View> {
            width: 28.0, height: 28.0,
            align: { x: 0.5, y: 0.5 },
            show_bg: true,
            draw_bg: {
                uniform border_color: (CRAWRAD_AVATAR_AGENT_BORDER)
                color: (CRAWRAD_AVATAR_AGENT)
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.circle(14.0, 14.0, 13.5);
                    sdf.fill_keep(self.color);
                    sdf.stroke(self.border_color, 1.0);
                    return sdf.result;
                }
            }
            r_avatar_glyph = <CrawradLabel> {
                width: Fit, height: Fit,
                align: { x: 0.5, y: 0.5 },
                draw_text: {
                    color: (CRAWRAD_ACENTO_TEXTO),
                    text_style: <THEME_FONT_REGULAR> { font_size: 13.0 }
                },
                text: "★"
            }
        }
        r_card_body = <CrawradCard> {
            width: Fill, height: Fit,

            r_sender_row = <View> {
                width: Fill, height: Fit,
                flow: Right, align: { y: 0.5 },
                r_sender_name = <CrawradLabel> {
                    draw_text: { color: (CRAWRAD_ACENTO_TEXTO) },
                    text: "Copilot"
                }
            }
            // The turn's final status line, dimmed: grey bar, secondary text.
            r_status_row = <CrawradAiStatusRow> {
                visible: false, margin: { top: (CRAWRAD_ESPACO_XS) }, draw_bg: { color: (CRAWRAD_TEXTO_TERCIARIO) }
                st_line = { draw_text: { color: (CRAWRAD_TEXTO_SECUNDARIO) } }
            }
            // A `Label` has no `visible`: the row around it is what hides.
            r_notice_row = <View> {
                visible: false,
                width: Fill, height: Fit,
                r_notice = <CrawradLabel> {
                    width: Fill,
                    draw_text: {
                        color: (CRAWRAD_TEXTO_TERCIARIO),
                        text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_CORPO) }
                    },
                    text: ""
                }
            }
            r_tool_card = <CrawradAiToolCard> {}
            r_blocks = <CrawradSegmentList> {
                text_template: <CrawradChatMarkdown> { margin: { top: (CRAWRAD_ESPACO_S) } }
                block_template: <CrawradAiCodeBlock> { visible: true }
            }
            r_meta_row = <View> {
                width: Fill, height: Fit,
                flow: Right, align: { x: 1.0, y: 0.5 }, spacing: (CRAWRAD_ESPACO_S),
                r_copy = <CrawradButton> { height: 18.0, text: "Copy text", draw_text: { text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_PEQUENO) } } }
                r_meta = <CrawradLabel> {
                    draw_text: {
                        color: (CRAWRAD_TEXTO_TERCIARIO),
                        text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_CORPO) }
                    },
                    text: ""
                }
            }
        }
    }

    pub CrawradAiThinkingCard = <CrawradCard> { visible: false, th_status = <CrawradAiStatusRow> {} }

    pub CrawradAiPlanCard = <CrawradCard> {
        visible: false,
        width: Fill, height: Fit,
        flow: Down,
        spacing: (CRAWRAD_ESPACO_S),

        plan_hdr_row = <View> {
            width: Fill, height: Fit,
            flow: Right, align: { y: 0.5 },
            plan_badge = <CrawradLabel> {
                draw_text: {
                    text_style: <THEME_FONT_BOLD> { font_size: 13.0 },
                    color: (CRAWRAD_ACENTO_TEXTO)
                },
                text: "⚡ Plan: Implementation"
            }
            plan_spacer = <View> { width: Fill, height: 1.0 }
            plan_status = <CrawradLabelMuted> {
                margin: { left: (CRAWRAD_ESPACO_M) },
                draw_text: { color: (CRAWRAD_AVISO) },
                text: "Pending Review"
            }
        }

        plan_timer_row = <View> {
            width: Fill, height: Fit,
            flow: Right, align: { y: 0.5 },
            plan_timer_label = <CrawradLabelMuted> {
                draw_text: { color: (CRAWRAD_TEXTO_TERCIARIO) },
                text: "Worked for 55s >"
            }
        }

        plan_title = <CrawradLabel> {
            width: Fill,
            draw_text: {
                text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_CORPO) },
                color: (CRAWRAD_TEXTO_PRIMARIO)
            },
            text: "Implementation Plan: Reverse Reach"
        }

        plan_body = <CrawradLabel> {
            width: Fill,
            draw_text: { color: (CRAWRAD_TEXTO_SECUNDARIO) },
            text: "• Invert the dependency graph\n• Resolve interface / implementation edges\n• Isolate the minimal build paths"
        }

        plan_actions_row = <View> {
            width: Fill, height: Fit,
            flow: Right, align: { y: 0.5 },
            spacing: (CRAWRAD_ESPACO_S),
            margin: { top: (CRAWRAD_ESPACO_XS) },

            btn_plan_approve = <CrawradButton> {
                height: 24.0,
                draw_text: { text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_CORPO) } },
                text: "✓ Approve"
            }
            btn_plan_reject = <CrawradButton> {
                height: 24.0,
                text: "✕ Reject"
            }
            plan_act_spacer = <CrawradFiller> {}
            btn_plan_auto_approve = <CrawradButton> {
                height: 24.0,
                text: "Auto-approve: OFF"
            }
        }
    }

    pub CrawradAiPermissionCard = <CrawradCard> {
        visible: false,
        width: Fill, height: Fit,
        flow: Down,
        spacing: (CRAWRAD_ESPACO_S),

        perm_hdr_row = <View> {
            width: Fill, height: Fit,
            flow: Right, align: { y: 0.5 },
            perm_badge = <CrawradLabel> {
                draw_text: {
                    text_style: <THEME_FONT_BOLD> { font_size: 13.0 },
                    color: (CRAWRAD_AVISO)
                },
                text: "🛡️ Permission Request"
            }
            perm_count = <CrawradLabelMuted> {
                margin: { left: (CRAWRAD_ESPACO_XS) },
                draw_text: { color: (CRAWRAD_TEXTO_SECUNDARIO) },
                text: ""
            }
            perm_spacer = <View> { width: Fill, height: 1.0 }
            perm_timeout = <CrawradLabelMuted> {
                draw_text: { color: (CRAWRAD_TEXTO_TERCIARIO) },
                text: "60s"
            }
        }

        perm_tool_row = <View> {
            width: Fill, height: Fit,
            flow: Right, align: { y: 0.5 },
            perm_tool_badge = <CrawradLabel> {
                draw_text: {
                    text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_CORPO) },
                    color: (CRAWRAD_ACENTO_TEXTO)
                },
                text: "Tool:"
            }
            lbl_perm_tool_name = <CrawradLabel> {
                margin: { left: (CRAWRAD_ESPACO_XS) },
                draw_text: {
                    text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_CORPO) },
                    color: (CRAWRAD_TEXTO_PRIMARIO)
                },
                text: ""
            }
        }

        perm_target_row = <View> {
            visible: true,
            width: Fill, height: Fit,
            flow: Right, align: { y: 0.5 },
            perm_target_badge = <CrawradLabelMuted> {
                text: "Target:"
            }
            lbl_perm_target = <CrawradLabel> {
                margin: { left: (CRAWRAD_ESPACO_XS) },
                draw_text: { color: (CRAWRAD_TEXTO_SECUNDARIO) },
                text: ""
            }
        }
        lbl_perm_args = <CrawradLabelMuted> {
            width: Fill, height: Fit,
            text: ""
        }

        perm_actions_row = <View> {
            width: Fill, height: Fit,
            flow: Right, align: { y: 0.5 },
            spacing: (CRAWRAD_ESPACO_S),
            margin: { top: (CRAWRAD_ESPACO_XS) },

            btn_perm_allow_once = <CrawradPrimaryButton> {
                height: 24.0,
                draw_text: { text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_CORPO) } },
                text: "Allow Once"
            }
            btn_perm_allow_always = <CrawradButton> {
                height: 24.0,
                text: "Allow Always"
            }
            btn_perm_reject_once = <CrawradButton> {
                height: 24.0,
                text: "Deny"
            }
            btn_perm_reject_always = <CrawradButton> {
                height: 24.0,
                text: "Deny Always"
            }
        }
    }

    pub CrawradAiQueueItemRow = <View> {
        visible: false,
        width: Fill, height: Fit,
        flow: Right, align: { y: 0.5 },
        padding: { left: 4.0, right: 4.0, top: 4.0, bottom: 4.0 },
        spacing: 8.0,

        // Thumbnail or icon on the left
        q_item_thumb = <CrawradOutlined> {
            width: 20.0, height: 20.0,
            align: { x: 0.5, y: 0.5 },
            draw_bg: {
                uniform border_radius: 3.0
                color: (CRAWRAD_FLUTUANTE)
            }
            q_thumb_icon = <CrawradLabelMuted> {
                draw_text: { text_style: <THEME_FONT_REGULAR> { font_size: 8.5 } }
                text: "💬"
            }
        }

        // Queued message text
        q_item_text = <CrawradLabel> {
            width: Fill, height: Fit,
            draw_text: {
                text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_CORPO) }
                color: (CRAWRAD_TEXTO_SECUNDARIO)
            }
            text: ""
        }

        // Message actions: Send Now (→), Edit (✎), Delete (🗑)
        q_item_actions = <View> {
            width: Fit, height: Fit,
            flow: Right, align: { y: 0.5 },
            spacing: 2.0,

            btn_q_send = <CrawradIconButton> {
                width: 22.0, height: 22.0,
                text: ""
            }
            btn_q_edit = <CrawradIconButton> {
                width: 22.0, height: 22.0,
                text: ""
            }
            btn_q_del = <CrawradIconButton> {
                width: 22.0, height: 22.0,
                text: ""
            }
        }
    }

    pub CrawradAiAttachChip = <CrawradOutlined> {
        visible: false,
        width: Fit, height: Fit,
        flow: Right, align: { y: 0.5 },
        padding: { left: 8.0, right: 4.0, top: 2.0, bottom: 2.0 },
        spacing: 4.0,
        draw_bg: {
            uniform border_radius: (CRAWRAD_RAIO)
            color: (CRAWRAD_FLUTUANTE)
        }
        att_text = <CrawradLabel> {
            draw_text: {
                text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_PEQUENO) }
                color: (CRAWRAD_TEXTO_SECUNDARIO)
            }
            text: ""
        }
        btn_att_remove = <CrawradPanelClose> {
            width: 16.0, height: 16.0,
            margin: 0.,
            text: ""
        }
    }

    pub CrawradAiQueueCard = <CrawradOutlined> {
        visible: false,
        width: Fill, height: Fit,
        flow: Down,
        margin: { left: (CRAWRAD_MARGEM_PAINEL), right: (CRAWRAD_MARGEM_PAINEL), top: 4.0, bottom: 4.0 },
        padding: { left: 10.0, right: 10.0, top: 8.0, bottom: 8.0 },
        spacing: 6.0,

        // Header: Queued Messages [3] Sends after agent finishes working  ˅
        queue_header = <View> {
            width: Fill, height: Fit,
            flow: Right, align: { y: 0.5 },
            spacing: 6.0,

            lbl_queue_title = <CrawradLabel> {
                draw_text: {
                    text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_CORPO) }
                    color: (CRAWRAD_TEXTO_PRIMARIO)
                }
                text: "Queued Messages"
            }

            badge_queue_count = <View> {
                width: Fit, height: 18.0,
                padding: { left: 6.0, right: 6.0 },
                align: { y: 0.5 },
                show_bg: true,
                draw_bg: {
                    uniform border_radius: 9.0
                    color: (CRAWRAD_SELECAO_FORTE)
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, self.border_radius);
                        sdf.fill(self.color);
                        return sdf.result;
                    }
                }
                lbl_queue_count = <CrawradLabel> {
                    draw_text: {
                        text_style: <THEME_FONT_BOLD> { font_size: 9.0 }
                        color: (CRAWRAD_TEXTO_PRIMARIO)
                    }
                    text: "0"
                }
            }
            lbl_queue_desc = <CrawradLabelMuted> {
                draw_text: { text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_PEQUENO) } }
                text: "Sends after agent finishes working"
            }

            queue_hdr_spacer = <CrawradFiller> {}
            btn_queue_toggle = <CrawradIconButton> {
                width: 22.0, height: 22.0,
                text: ""
            }
        }

        // List of queued items
        queue_items = <View> {
            width: Fill, height: Fit,
            flow: Down,
            spacing: 4.0,

            q_item_0 = <CrawradAiQueueItemRow> {}
            q_item_1 = <CrawradAiQueueItemRow> {}
            q_item_2 = <CrawradAiQueueItemRow> {}
            q_item_3 = <CrawradAiQueueItemRow> {}
            q_item_4 = <CrawradAiQueueItemRow> {}
            q_item_5 = <CrawradAiQueueItemRow> {}
            q_item_6 = <CrawradAiQueueItemRow> {}
            q_item_7 = <CrawradAiQueueItemRow> {}
        }
    }

    // The circular context button (+) in the AI prompt.
    pub CrawradContextButton = <CrawradCircleButton> {
        width: 24.0, height: 24.0,
        draw_bg: {
            color: (CRAWRAD_CONTROLE)
            color_hover: (CRAWRAD_SELECAO_FORTE)
            color_down: (CRAWRAD_SELECAO)
        }
        draw_text: {
            text_style: <THEME_FONT_BOLD> { font_size: 12.0 }
            color: (CRAWRAD_TEXTO_PRIMARIO)
        }
    }

    pub CrawradAiInputBar = <CrawradOutlined> {
        width: Fill, height: Fit,
        flow: Down,
        margin: { left: (CRAWRAD_MARGEM_PAINEL), right: (CRAWRAD_MARGEM_PAINEL), top: (CRAWRAD_ESPACO_S), bottom: (CRAWRAD_MARGEM_PAINEL) },
        padding: { left: 12.0, right: 12.0, top: 8.0, bottom: 8.0 },
        spacing: 6.0,

        ai_attach_chips = <View> {
            visible: false,
            width: Fill, height: Fit,
            flow: Right, spacing: (CRAWRAD_ESPACO_S),
            att_0 = <CrawradAiAttachChip> {}
            att_1 = <CrawradAiAttachChip> {}
            att_2 = <CrawradAiAttachChip> {}
            att_3 = <CrawradAiAttachChip> {}
        }

        prompt_input = <CrawradPromptInput> {
            width: Fill,
            height: 38.0,
            empty_text: "Ask anything, @ to mention, / for actions"
        }

        ai_tools_bar = <View> {
            width: Fill, height: Fit,
            flow: Right,
            spacing: (CRAWRAD_ESPACO_S),
            align: { y: 0.5 },

            btn_ai_attach = <CrawradContextButton> { text: "+" }
            btn_ai_model = <CrawradButton> { height: 22.0, text: "Sonnet 3.7 ▾" }
            btn_ai_effort = <CrawradButton> { visible: false, height: 22.0, text: "" }
            ai_tools_spacer = <CrawradFiller> {}
            btn_ai_mic = <CrawradMicButton> { text: "" }
            btn_prompt_send = <CrawradCircleButton> { text: "→" }
            // Stop, as the mockup draws it: a dark circle, a small white square.
            btn_prompt_stop = <CrawradCircleButton> { visible: false, text: "■",
                draw_bg: { color: (CRAWRAD_AVATAR_USER), color_hover: (CRAWRAD_SELECAO_FORTE) },
                draw_text: { text_style: { font_size: 6.5 }, color: (CRAWRAD_BRANCO) } }
        }
    }

    // Context Popup (+): Media, Mentions, Actions, Browser, and Tools (Ref: media_1790351098625.png)
    pub CrawradAiContextPopup = <CrawradOutlined> {
        visible: false,
        width: 210.0, height: Fit,
        flow: Down,
        margin: { left: 14.0, top: 400.0 },
        padding: 8.0,
        draw_bg: {
            color: (CRAWRAD_FLUTUANTE)
        }

        ctx_header = <View> {
            width: Fill, height: Fit,
            flow: Right, align: { y: 0.5 },
            margin: { left: 4.0, bottom: 6.0, top: 2.0 },
            lbl_ctx_title = <CrawradLabelMuted> {
                draw_text: { text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_PEQUENO) } },
                text: "Add Context"
            }
        }

        ctx_list = <View> {
            width: Fill, height: Fit,
            flow: Down,
            spacing: 2.0,

            btn_ai_media = <CrawradButton> { align: { x: 0.0, y: 0.5 }, padding: { left: 8.0, right: 8.0 }, width: Fill, height: 26.0, text: "🖼   Media" }
            btn_ai_mentions = <CrawradButton> { align: { x: 0.0, y: 0.5 }, padding: { left: 8.0, right: 8.0 }, width: Fill, height: 26.0, text: "@   Mentions" }
            btn_ai_actions = <CrawradButton> { align: { x: 0.0, y: 0.5 }, padding: { left: 8.0, right: 8.0 }, width: Fill, height: 26.0, text: "☑   Actions" }
            sep_ctx = <CrawradSeparatorH> { margin: { top: 4.0, bottom: 4.0 } }
            btn_ai_memory = <CrawradButton> { align: { x: 0.0, y: 0.5 }, padding: { left: 8.0, right: 8.0 }, width: Fill, height: 26.0, text: "🧠   Memory" }
            btn_ai_mcp = <CrawradButton> { align: { x: 0.0, y: 0.5 }, padding: { left: 8.0, right: 8.0 }, width: Fill, height: 26.0, text: "🔌   MCP Tools" }
            btn_ai_agent = <CrawradButton> { align: { x: 0.0, y: 0.5 }, padding: { left: 8.0, right: 8.0 }, width: Fill, height: 26.0, text: "🤖   Agents" }
        }
    }

    pub CrawradAiSlashPopup = <CrawradOutlined> {
        visible: false,
        width: Fill, height: Fit,
        flow: Down,
        margin: { left: (CRAWRAD_MARGEM_PAINEL), right: (CRAWRAD_MARGEM_PAINEL), top: 380.0 },
        padding: (CRAWRAD_MARGEM_PAINEL),
        draw_bg: {
            color: (CRAWRAD_FLUTUANTE)
        }

        popup_header = <View> {
            width: Fill, height: Fit,
            flow: Right, align: { y: 0.5 },
            margin: { bottom: (CRAWRAD_ESPACO_S) },
            lbl_cmd_title = <CrawradLabelMuted> {
                draw_text: { text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_PEQUENO) } },
                text: ">_ COMMANDS"
            }
        }

        commands_list = <View> {
            width: Fill, height: Fit,
            flow: Down,
            spacing: (CRAWRAD_ESPACO_XS),

            btn_cmd_explain = <CrawradButton> { align: { x: 0.0, y: 0.5 }, padding: { left: 8.0, right: 8.0 }, width: Fill, height: 26.0, text: ">_  /explain       Explain code or selection" }
            btn_cmd_refactor = <CrawradButton> { align: { x: 0.0, y: 0.5 }, padding: { left: 8.0, right: 8.0 }, width: Fill, height: 26.0, text: ">_  /refactor      Refactor and optimize code" }
            btn_cmd_test = <CrawradButton> { align: { x: 0.0, y: 0.5 }, padding: { left: 8.0, right: 8.0 }, width: Fill, height: 26.0, text: ">_  /test          Generate unit tests" }
            btn_cmd_docs = <CrawradButton> { align: { x: 0.0, y: 0.5 }, padding: { left: 8.0, right: 8.0 }, width: Fill, height: 26.0, text: ">_  /docs          Generate documentation" }
            btn_cmd_find = <CrawradButton> { align: { x: 0.0, y: 0.5 }, padding: { left: 8.0, right: 8.0 }, width: Fill, height: 26.0, text: ">_  /find          Find symbols or usage" }
            btn_cmd_memory = <CrawradButton> { align: { x: 0.0, y: 0.5 }, padding: { left: 8.0, right: 8.0 }, width: Fill, height: 26.0, text: ">_  /memory        Open global memory dialog" }
            btn_cmd_mcp = <CrawradButton> { align: { x: 0.0, y: 0.5 }, padding: { left: 8.0, right: 8.0 }, width: Fill, height: 26.0, text: ">_  /mcp           Open MCP servers dialog" }
            btn_cmd_new = <CrawradButton> { align: { x: 0.0, y: 0.5 }, padding: { left: 8.0, right: 8.0 }, width: Fill, height: 26.0, text: ">_  /new           Create project scaffold" }
        }

        popup_footer = <View> {
            width: Fill, height: Fit,
            flow: Right, align: { y: 0.5 },
            margin: { top: (CRAWRAD_ESPACO_S) },
            lbl_cmd_footer = <CrawradLabelMuted> {
                draw_text: { text_style: { font_size: (CRAWRAD_FONTE_PEQUENO) } },
                text: "⇅ navigate   Enter run   Esc close"
            }
        }
    }

    pub CrawradAiModelDropdown = <CrawradOutlined> {
        visible: false,
        width: 180.0, height: Fit,
        flow: Down,
        spacing: 2.0,
        margin: { left: 44.0, top: 400.0 },
        padding: 6.0,
        draw_bg: {
            uniform border_color: (CRAWRAD_SELECAO_FORTE)
            color: (CRAWRAD_FLUTUANTE)
        }
        menu_title = <CrawradLabelMuted> { margin: { left: 4.0, bottom: 4.0, top: 2.0 }, text: "", draw_text: { text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_PEQUENO) } } }
        opt_0 = <CrawradMenuSlot> {} opt_1 = <CrawradMenuSlot> {} opt_2 = <CrawradMenuSlot> {} opt_3 = <CrawradMenuSlot> {}
        opt_4 = <CrawradMenuSlot> {} opt_5 = <CrawradMenuSlot> {} opt_6 = <CrawradMenuSlot> {} opt_7 = <CrawradMenuSlot> {}
        opt_8 = <CrawradMenuSlot> {} opt_9 = <CrawradMenuSlot> {} opt_10 = <CrawradMenuSlot> {} opt_11 = <CrawradMenuSlot> {}
        opt_12 = <CrawradMenuSlot> {} opt_13 = <CrawradMenuSlot> {} opt_14 = <CrawradMenuSlot> {} opt_15 = <CrawradMenuSlot> {}
    }

    // One past conversation in the history list (board 4 of the approved mockup)
    pub CrawradHistoryRow = <View> {
        visible: false, width: Fill, height: Fit, flow: Down,
        // a Label cannot hide itself: its box does
        h_group_box = <View> { width: Fill, height: Fit, h_group = <CrawradLabelMuted> { margin: { left: 6.0, top: 8.0, bottom: 3.0 }, text: "", draw_text: { text_style: { font_size: 8.5 } } } }
        h_item = <View> {
            width: Fill, height: 38.0, flow: Overlay,
            h_now = <View> { visible: false, width: Fill, height: Fill, show_bg: true, draw_bg: { color: (CRAWRAD_SELECAO) } }
            h_hit = <CrawradButton> { width: Fill, height: Fill, text: "", draw_bg: {
                color: (CRAWRAD_TRANSPARENTE), color_hover: (CRAWRAD_SELECAO), color_down: (CRAWRAD_SELECAO_FORTE), color_focus: (CRAWRAD_TRANSPARENTE),
                border_color_1: (CRAWRAD_TRANSPARENTE), border_color_2: (CRAWRAD_TRANSPARENTE), border_color_1_hover: (CRAWRAD_TRANSPARENTE), border_color_2_hover: (CRAWRAD_TRANSPARENTE),
                border_color_1_down: (CRAWRAD_TRANSPARENTE), border_color_2_down: (CRAWRAD_TRANSPARENTE), border_color_1_focus: (CRAWRAD_TRANSPARENTE), border_color_2_focus: (CRAWRAD_TRANSPARENTE) } }
            <View> {
                width: Fill, height: Fill, flow: Right, spacing: 8.0, padding: { left: 6.0, right: 6.0, top: 4.0, bottom: 4.0 },
                h_bar = <View> { visible: false, width: 3.0, height: Fill, show_bg: true, draw_bg: { color: (CRAWRAD_ACENTO) } }
                <View> { width: Fill, height: Fill, flow: Down, spacing: 2.0, align: { y: 0.5 },
                    h_title = <CrawradLabel> { text: "" }
                    h_meta = <CrawradLabelMuted> { text: "", draw_text: { text_style: { font_size: 8.5 } } } }
                <View> { width: Fit, height: Fill, flow: Down, spacing: 2.0, align: { x: 1.0, y: 0.5 },
                    h_when = <CrawradLabelMuted> { text: "", draw_text: { text_style: { font_size: 8.5 } } }
                    h_resume = <CrawradLabelMuted> { text: "", draw_text: { text_style: { font_size: 8.0 } } } }
            }
        }
    }

    pub CrawradAiHistoryPopup = <View> {
        visible: false, width: 330.0, height: Fit, flow: Down, padding: 6.0,
        show_bg: true,
        draw_bg: {
            uniform border_color: (CRAWRAD_SELECAO_FORTE)
            color: (CRAWRAD_FLUTUANTE)
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, 4.0);
                sdf.fill_keep(self.color);
                sdf.stroke(self.border_color, 1.0);
                return sdf.result;
            }
        }
        <View> { width: Fill, height: Fit, flow: Right, padding: { left: 4.0, right: 4.0, top: 2.0, bottom: 6.0 },
            <CrawradLabel> { text: "History", draw_text: { text_style: <THEME_FONT_BOLD> { font_size: (CRAWRAD_FONTE_PEQUENO) } } }
            <CrawradFiller> { height: Fit }
            hist_count = <CrawradLabelMuted> { text: "", draw_text: { text_style: { font_size: 8.5 } } } }
        hist_search = <CrawradTextInput> { width: Fill, empty_text: "Search conversations" }
        hist_empty = <View> { visible: false, width: Fill, height: Fit, <CrawradLabelMuted> { margin: { left: 6.0, top: 10.0, bottom: 8.0 }, text: "No conversations yet" } }
        hist_0 = <CrawradHistoryRow> {} hist_1 = <CrawradHistoryRow> {} hist_2 = <CrawradHistoryRow> {} hist_3 = <CrawradHistoryRow> {}
        hist_4 = <CrawradHistoryRow> {} hist_5 = <CrawradHistoryRow> {} hist_6 = <CrawradHistoryRow> {} hist_7 = <CrawradHistoryRow> {}
        hist_8 = <CrawradHistoryRow> {} hist_9 = <CrawradHistoryRow> {}
    }



    pub CrawradPanelChat = <CrawradPanel> {
        width: Fill, height: Fill,
        flow: Overlay,

        chat_base = <View> {
            width: Fill, height: Fill,
            flow: Down,

            ai_header = <CrawradPanelHeader> {
                title = { text: "Aefos AI" }
                ai_mock_badge = <CrawradOutlined> {
                    width: Fit, height: 18.0,
                    margin: { left: (CRAWRAD_ESPACO_S) },
                    padding: { left: (CRAWRAD_ESPACO_S), right: (CRAWRAD_ESPACO_S) },
                    align: { y: 0.5 },
                    draw_bg: {
                        uniform border_radius: (CRAWRAD_RAIO)
                        color: (CRAWRAD_SELECAO)
                    }
                    ai_mock_text = <CrawradLabelMuted> {
                        draw_text: { color: (CRAWRAD_AVISO) },
                        text: "No agent"
                    }
                }
                ai_hdr_spacer = <CrawradFiller> {}
                btn_ai_new = <CrawradIconButton> { text: "" }
                btn_ai_clear = <CrawradIconButton> { margin: { left: (CRAWRAD_ESPACO_XS) }, text: "" }
                btn_ai_history = <CrawradIconButton> { margin: { left: (CRAWRAD_ESPACO_XS) }, text: "" }
                btn_ai_settings = <CrawradIconButton> { margin: { left: (CRAWRAD_ESPACO_XS) }, text: "" }
                btn_panel_close = <CrawradPanelClose> { text: "" }
            }

            sep_ai_header = <CrawradSeparatorH> {}

            // Toolbar strip: Form: ON | Sel: ON
            ai_toggles_bar = <View> {
                width: Fill, height: (CRAWRAD_ABA_ALTURA),
                flow: Right,
                padding: { left: (CRAWRAD_MARGEM_PAINEL), right: (CRAWRAD_MARGEM_PAINEL) },
                align: { y: 0.5 },
                spacing: (CRAWRAD_ESPACO_S),

                btn_form_toggle = <CrawradButton> {
                    height: 20.0,
                    padding: { left: 8.0, right: 8.0 },
                    draw_text: { text_style: { font_size: (CRAWRAD_FONTE_PEQUENO) } },
                    text: "Form: ON"
                }
                btn_sel_toggle = <CrawradButton> {
                    height: 20.0,
                    padding: { left: 8.0, right: 8.0 },
                    draw_text: { text_style: { font_size: (CRAWRAD_FONTE_PEQUENO) } },
                    text: "Sel: ON"
                }
            }

            sep_ai_toggles = <CrawradSeparatorH> {}

            // Who answers, and how: ACP with its session, or the CLI
            // fallback. Outside the scroll, so it is always in sight.
            ai_agent_chip_row = <View> {
                visible: false,
                width: Fill, height: Fit,
                align: { x: 0.5 },
                padding: { top: 6.0, bottom: 2.0 },
                ai_agent_chip = <View> {
                    width: Fit, height: 24.0,
                    flow: Overlay,
                    chip_face = <View> {
                        width: Fit, height: 24.0,
                        flow: Right, align: { y: 0.5 },
                        spacing: 6.0,
                        padding: { left: 9.0, right: 11.0 },
                        show_bg: true,
                        draw_bg: {
                            uniform border_color: (CRAWRAD_DIVISOR)
                            color: (CRAWRAD_PAINEL)
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, 4.0);
                                sdf.fill_keep(self.color);
                                sdf.stroke(self.border_color, 1.0);
                                return sdf.result;
                            }
                        }
                        chip_dot_waiting = <CrawradChipDot> { draw_bg: { color: (CRAWRAD_TEXTO_TERCIARIO) } }
                        chip_dot_acp = <CrawradChipDot> { draw_bg: { color: (CRAWRAD_CHIP_ACP) } }
                        chip_dot_cli = <CrawradChipDot> { draw_bg: { color: (CRAWRAD_AVISO) } }
                        chip_dot_failed = <CrawradChipDot> { draw_bg: { color: (CRAWRAD_ERRO) } }
                        chip_text = <CrawradLabel> { width: Fit, text: "",
                            draw_text: { color: (CRAWRAD_TEXTO_SECUNDARIO), text_style: <THEME_FONT_REGULAR> { font_size: (CRAWRAD_FONTE_PEQUENO) } } }
                    }
                    btn_chip = <CrawradListRowOverlay> { width: Fill, height: Fill }
                }
            }

            // Chat messages viewport
            ai_messages_view = <View> {
                width: Fill, height: Fill,
                flow: Down,
                padding: (CRAWRAD_MARGEM_PAINEL),
                spacing: (CRAWRAD_ESPACO_M),
                scroll_bars: <CrawradScrollBars> {}

                ai_hero_view = <CrawradAiHeroView> {}
                copilot_card = <CrawradAiCopilotCard> {}
                user_card = <CrawradAiUserCard> {}
                reply_card = <CrawradAiReplyCard> {}
                user_card_1 = <CrawradAiUserCard> {}
                reply_card_1 = <CrawradAiReplyCard> {}
                user_card_2 = <CrawradAiUserCard> {}
                reply_card_2 = <CrawradAiReplyCard> {}
                user_card_3 = <CrawradAiUserCard> {}
                reply_card_3 = <CrawradAiReplyCard> {}
                user_card_4 = <CrawradAiUserCard> {}
                reply_card_4 = <CrawradAiReplyCard> {}
                user_card_5 = <CrawradAiUserCard> {}
                reply_card_5 = <CrawradAiReplyCard> {}
                user_card_6 = <CrawradAiUserCard> {}
                reply_card_6 = <CrawradAiReplyCard> {}
                user_card_7 = <CrawradAiUserCard> {}
                reply_card_7 = <CrawradAiReplyCard> {}
                user_card_8 = <CrawradAiUserCard> {}
                reply_card_8 = <CrawradAiReplyCard> {}
                user_card_9 = <CrawradAiUserCard> {}
                reply_card_9 = <CrawradAiReplyCard> {}
                user_card_10 = <CrawradAiUserCard> {}
                reply_card_10 = <CrawradAiReplyCard> {}
                user_card_11 = <CrawradAiUserCard> {}
                reply_card_11 = <CrawradAiReplyCard> {}
                user_card_12 = <CrawradAiUserCard> {}
                reply_card_12 = <CrawradAiReplyCard> {}
                user_card_13 = <CrawradAiUserCard> {}
                reply_card_13 = <CrawradAiReplyCard> {}
                user_card_14 = <CrawradAiUserCard> {}
                reply_card_14 = <CrawradAiReplyCard> {}
                user_card_15 = <CrawradAiUserCard> {}
                reply_card_15 = <CrawradAiReplyCard> {}
                thinking_card = <CrawradAiThinkingCard> {}
                plan_card = <CrawradAiPlanCard> {}
                permission_card = <CrawradAiPermissionCard> {}
                chat_bottom_spacer = <View> { width: Fill, height: 16.0 }
            }

            // Send queue (Queued Messages)
            ai_queue_card = <CrawradAiQueueCard> {}

            // Chat prompt container: unified floating card (modern-editor style)
            ai_input_bar = <CrawradAiInputBar> {}
        }

        // Floating Context Popup (+ button: Media, Mentions, Actions, Browser, etc.)
        ai_context_popup = <CrawradAiContextPopup> {}

        // Floating Slash Commands Popup (Ref 08: 08_slash_commands_popup.png)
        ai_slash_popup = <CrawradAiSlashPopup> {}

        // Model Selector Dropdown (Ref 09: 09_model_selector_dropdown.png)
        ai_model_dropdown = <CrawradAiModelDropdown> {}

        // The conversation history, under the header's clock button
        ai_history_popup = <CrawradAiHistoryPopup> {}
    }


}
