```
aardwolf-templates/
├── compiled_templates                      -- This is where the rust-compiled templates end up
│   └── templates                           -- Follows the same structure as the HTML/Ructe path
│       ├── home                            -- As below
│       └── ui                              -- As below
├── src                                     -- Not documented yet...
│   └── ui                                  -- Not documented yet...
│
├── templates                               -- The root for the HTML/Ructe templates
│   │
│   ├── asides                              -- Left-hand navigation menus 
│   │   ├── aside_settings.rs.html              -- Settings menu
│   │   └── aside_shortcuts.rs.html             -- Shortcuts menu
│   │
│   ├── containers                          -- Main container layouts
│   │   ├── container_preferences.rs.html       -- Preferences
│   │   └── container_profile.rs.html           -- Profile 
│   │
│   ├── elements                            -- Misc UI elements
│   │   ├── alert.rs.html                       -- Alerts
│   │   ├── icon.rs.html                        -- Icon
│   │   ├── input.rs.html                       -- Visible input
│   │   ├── password_input.rs.html              -- Hidden input
│   │   └── text_input.rs.html                  -- Text input
│   │
│   ├── email                               -- Templates used when sending e-mails
│   │   └── new_user_welcome.rs.html            -- Welcome email
│   │
│   ├── error                               -- Error pages 
│   │   └── http_error.html                     -- Basic Error page
│   │  
│   ├── home                                -- Homepage layouts
│   │   ├── home_feed.rs.html                   -- Home feed
│   │   ├── home_footer.rs.html                 -- Footer for logged in user
│   │   └── home_nav_top.rs.html                -- Top navigation for logged in user
│   │
│   ├── posts                               -- Templates related to posts
│   │   └── new_post.rs.html                    -- New post
│   │
│   ├── reply                               -- Reply layouts
│   │   └── reply_box.rs.html                   -- Base template for posting a reply 
│   │
│   ├── base_template.html                      -- Base template
│   ├── html_head.rs.html                       -- This is the <head> content
│   ├── sign_in.rs.html                         -- User sign-in page
│   └── sign_up.rs.html                         -- User sign-up page
│
├── build.rs                                -- Template build instructions
└── Cargo.toml                              -- Cargo crate whoozits
```
