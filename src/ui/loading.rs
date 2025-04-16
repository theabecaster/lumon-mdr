use ratatui::{
    Frame,
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::Paragraph,
};

use crate::app::App;

// Lumon logo ASCII art
const LUMON_LOGO: &[&str] = &[
    "                                               ZTMIKMLKOVSRSY   ZSSTZTNIHHKQX                                          ",
    "                                         WPNPWZVZ       VP  Y   Z  OX       NSYTNOU                                    ",
    "                                     XRRUW   TTP  KKMIGN NRSY   ZSSO XGDHHI  ZRY  UWQQU                                ",
    "                                  ZSLD  EDCBCL CCCCFGD FCCGGGGGGGGGDCC CGGECCCD BCCD  CGMT                             ",
    "                                TTS                                                      NTT                           ",
    "                              YWQ VRLGGGFC CGGGGGGGFLUEGGGGGGGGGGGGGGGFI DGGGGGGGGC CFGGMN OTT                         ",
    "                             VT  RT     ZXTT       Z YY               ZW W        TSZZ   TTN MT                        ",
    "                             T SWY    TMMSZ    TMMTSMMT   TMLSXQKMT  TMFHHHGMT UNINMLLT    TM NU                       ",
    "                            WS T      M  M     N  MM  M   M  J J  M  M       M O   B  M     TH N                       ",
    "                            U RX      G  G     H  GG  G   G   U   G  G  CGE  G J      G      H H                       ",
    "                            U XS      L  L     M      M   L       L  L  BFB  L O  B   L     YL M                       ",
    "                            XQ MT     S     Z  TLZ   NS   S   M   V  R       S T  L   S    YV TT                       ",
    "                             TN OUZ   ZYRRRYZ   ZWUNPRZ   ZYYZ ZYYZ  ZRFCCGLSZ ZYYYVVVZ   TR  X                        ",
    "                              TO XSSW   T OUU      U NT               WP T       TSZ X  TTM PWY                        ",
    "                               VUQ  QRRXTTS OTU    UN NU             WS PT     TTM PWQSSS MTT                          ",
    "                                 WSSQ  VROLRX OQQW  TN NT           TP SW   URRP HJJVT  STT                            ",
    "                                    URRV   XQKC  TSOPPN NTT       TSM PVTONQK  KST  ZTSSZ                              ",
    "                                       YROQXX    T    LGD CGGGGGGGC HHNX        ZUPPVZ                                 ",
    "                                            XQMNTXSOOW                 XTONQPLNUZ                                      ",
    "                                                    ZXRLIHHGGGGGGGHHKOUY                                               ",
];

// Loading messages for the loading screen
pub const LOADING_MESSAGES: &[&str] = &[
    "Initializing MDR protocol",
    "Checking refinement quotas",
    "Verifying department credentials",
    "Preparing macrodata bins",
    "Establishing connection to Lumon mainframe",
    "Running compliance check",
    "Validating severance chip",
    "Please enjoy all amenities equally",
];

/// Renders the loading screen with logo and progress indicator
pub fn draw_loading_screen<B: Backend>(frame: &mut Frame<B>, area: Rect, app: &App) {
    // Render logo
    let logo_spans: Vec<Spans> = LUMON_LOGO
        .iter()
        .map(|&line| Spans::from(Span::styled(line, app.palette.fg_style())))
        .collect();

    let logo_w = LUMON_LOGO[0].len() as u16;
    let logo_h = LUMON_LOGO.len() as u16;
    let x = area.x + (area.width.saturating_sub(logo_w)) / 2;
    let y = area.y + (area.height.saturating_sub(logo_h)) / 2;
    let rect = Rect::new(x, y, logo_w, logo_h);

    let logo_para =
        Paragraph::new(logo_spans).style(app.palette.bg_style());
    frame.render_widget(logo_para, rect);

    // Render loading message and progress bar
    if area.height > logo_h + y + 3 {
        // Determine which message to show based on progress
        let total_messages = LOADING_MESSAGES.len();
        
        let message_idx = if app.progress_percentage >= 100.0 {
            total_messages - 1
        } else if total_messages > 1 {
            let scaled_progress = app.progress_percentage / 100.0 * (total_messages - 1) as f32;
            scaled_progress.floor() as usize
        } else {
            0
        };
        
        let message = LOADING_MESSAGES[message_idx];
    
        let message_span = Span::styled(
            message,
            Style::default().fg(Color::White),
        );
        
        // Layout for message and progress bar
        let vertical_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(y + logo_h + 2),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(2),
                Constraint::Min(0),
            ])
            .split(area);
            
        // Render message
        let message_para = Paragraph::new(Spans::from(message_span))
            .alignment(ratatui::layout::Alignment::Center);
        frame.render_widget(message_para, vertical_layout[1]);
        
        // Create progress bar
        let progress_width = vertical_layout[3].width.saturating_sub(15); // Make it less wide to leave room for percentage
        let filled = (progress_width as f32 * (app.progress_percentage / 100.0)) as u16;
        
        // Create a simple one-line progress bar
        let mut progress_bar = String::new();
        
        progress_bar.push('[');
        for i in 0..progress_width {
            if i < filled {
                progress_bar.push('=');
            } else {
                progress_bar.push(' ');
            }
        }
        progress_bar.push(']');
        
        // Add percentage at the end
        progress_bar.push_str(&format!(" {:3.0}%", app.progress_percentage));
        
        let progress_text = Paragraph::new(progress_bar)
            .alignment(ratatui::layout::Alignment::Center)
            .style(app.palette.fg_style());
        
        frame.render_widget(progress_text, vertical_layout[3]);
    }
} 