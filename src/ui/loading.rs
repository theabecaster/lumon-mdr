use ratatui::{
    Frame,
    backend::Backend,
    layout::Rect,
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
    "                            WS T      M  M     N  MM  M   M   U   M  M       M O   B  M     TH N                       ",
    "                            U RX      G  G     H  GG  G   G       G  G  CGE  G J      G      H H                       ",
    "                            U XS      L  L     M      M   L  U U  L  L  BFB  L O  B   L     YL M                       ",
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
    // Check if the area is large enough for the logo
    let logo_w = LUMON_LOGO[0].len() as u16;
    let logo_h = LUMON_LOGO.len() as u16;
    
    // Determine if we have enough space for the full logo
    let has_space_for_logo = area.width >= logo_w && area.height >= logo_h + 5; // +5 for progress bar
    
    if has_space_for_logo {
        // Render full logo
        let logo_spans: Vec<Spans> = LUMON_LOGO
            .iter()
            .map(|&line| Spans::from(Span::styled(line, app.palette.fg_style())))
            .collect();

        let x = area.x + (area.width.saturating_sub(logo_w)) / 2;
        let y = area.y + (area.height.saturating_sub(logo_h + 5)) / 2; // Center vertically accounting for progress bar
        let rect = Rect::new(x, y, logo_w.min(area.width), logo_h.min(area.height - 5));

        let logo_para = Paragraph::new(logo_spans).style(app.palette.bg_style());
        frame.render_widget(logo_para, rect);
        
        // Place progress indicator below logo
        let progress_y = y + logo_h + 2;
        if progress_y < area.y + area.height {
            draw_progress_indicator(frame, area, app, progress_y);
        }
    } else {
        // For small windows, show only text and progress bar
        let text = "LUMON INDUSTRIES";
        let text_spans = Spans::from(Span::styled(text, app.palette.fg_style()));
        
        let text_y = area.y + area.height / 3;
        let text_rect = Rect::new(area.x, text_y, area.width, 1);
        
        let text_para = Paragraph::new(text_spans)
            .alignment(ratatui::layout::Alignment::Center)
            .style(app.palette.fg_style());
        
        frame.render_widget(text_para, text_rect);
        
        // Place progress indicator in the middle
        let progress_y = text_y + 2;
        if progress_y < area.y + area.height {
            draw_progress_indicator(frame, area, app, progress_y);
        }
    }
}

/// Helper function to draw progress indicator
fn draw_progress_indicator<B: Backend>(frame: &mut Frame<B>, area: Rect, app: &App, y_position: u16) {
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
    
    // Message rect
    let message_rect = Rect::new(area.x, y_position, area.width, 1);
    
    // Progress bar rect
    let progress_rect = Rect::new(area.x, y_position + 1, area.width, 1);
    
    // Render message
    let message_para = Paragraph::new(Spans::from(message_span))
        .alignment(ratatui::layout::Alignment::Center);
    frame.render_widget(message_para, message_rect);
    
    // Create progress bar
    let progress_width = area.width.saturating_sub(15); // Make it less wide to leave room for percentage
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
    
    frame.render_widget(progress_text, progress_rect);
} 