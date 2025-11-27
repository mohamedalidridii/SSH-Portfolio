// ui/handler.rs

#[derive(Debug, Clone, PartialEq)]
pub enum Page {
    Home,
    Store,  // Projects page
    About,
    FAQ,    // Contact page
}

#[derive(Debug, Clone)]
pub struct UIState {
    pub current_page: Page,
    pub scroll_offset: usize,
    pub selected_item: usize,
}

impl UIState {
    pub fn new() -> Self {
        Self {
            current_page: Page::Home,
            scroll_offset: 0,
            selected_item: 0,
        }
    }

    pub fn reset_scroll(&mut self) {
        self.scroll_offset = 0;
        self.selected_item = 0;
    }
}

impl Default for UIState {
    fn default() -> Self {
        Self::new()
    }
}

pub struct PageContent;

impl PageContent {
    pub fn get_content(page: &Page) -> String {
        match page {
            Page::Home => Self::home_content(),
            Page::Store => Self::projects_content(),
            Page::About => Self::about_content(),
            Page::FAQ => Self::contact_content(),
        }
    }

    fn home_content() -> String {
        r#"
    ███╗   ███╗███████╗██████╗  █████╗ ██╗  ██╗   ██╗
    ████╗ ████║██╔════╝██╔══██╗██╔══██╗██║  ╚██╗ ██╔╝  
    ██╔████╔██║█████╗  ██║  ██║███████║██║   ╚████╔╝ 
    ██║╚██╔╝██║██╔══╝  ██║  ██║██╔══██║██║    ╚██╔╝  
    ██║ ╚═╝ ██║███████╗██████╔╝██║  ██║███████╗██║   
    ╚═╝     ╚═╝╚══════╝╚═════╝ ╚═╝  ╚═╝╚══════╝╚═╝

# Welcome to medaly.engineer

## Engineer | Content Creator | Founder @ KOYOTEC DIGITAL


  ╔════════════════════════════════════════════════════════════════════╗
  ║                                                                    ║
  ║   "Hey! new friend? (I use Arch btw)"                              ║
  ║   "Welcome to localhost community"                                 ║
  ║                                                                    ║
  ╚════════════════════════════════════════════════════════════════════╝


## About Me

Hi! I'm Med Ali Dridi, an IT Engineering student at ESPRIT University.
Originally a Mechatronics Engineer, I pivoted my career into Software 
Engineering & DevOps because I'm passionate about innovation, creativity,
and high-performance web experiences.


## What I Do

- 🚀 Build next-gen websites with cutting-edge tech
- 🎨 Create beautiful, animated web experiences
- ⚙️  DevOps & containerization enthusiast
- 📱 Content creation about tech & creativity
- 🏢 Run KOYOTEC DIGITAL - Top 1% creative agency in Tunisia


## Tech Stack

```rust
let skills = vec![
    // Frontend Magic
    "Next.js", "React", "GSAP", "Three.js",
    
    // DevOps & Infrastructure
    "Linux (Arch btw)", "Docker", "AKS", "CI/CD",
    
    // Languages
    "JavaScript/TypeScript", "Rust",
];
```


## KOYOTEC DIGITAL

We bring tech, design, and storytelling together. Part of the top 1% 
most creative agencies in Tunisia, we specialize in:
- High-performance web applications
- Stunning animations and 3D experiences
- Modern web architecture
- Creative solutions that feel like art


## Philosophy

I believe technology should always feel like art. That's what drives
every project at KOYOTEC and everything I create.


## Fun Facts

- 🕺 Often debugging at 1 a.m.
- 💬 Creating content & Reels about tech and DevOps
- 🌍 Based in Tunisia, building for the world
- 🧩 Every project is a canvas for innovation


Navigate using the menu above to explore my work!

Press 'a' for Projects | 's' for About | 'd' for Contact
"#.to_string()
    }

    fn projects_content() -> String {
        r#"
# My Projects

## Innovation Meets Creativity


## 🏢 KOYOTEC DIGITAL
## Tech: Next.js, React, GSAP, Three.js, Modern Web Stack

A top 1% creative digital agency based in Tunisia, building next-generation
websites that push the boundaries of what's possible on the web.

- High-performance web applications
- Cutting-edge animation techniques
- 3D interactive experiences
- Modern, responsive designs that feel alive
- Storytelling through technology

Our mission: Making technology feel like art, bringing together design,
tech, and narrative into unforgettable digital experiences.

```javascript
// Our approach
const koyotec = {
  creativity: "top 1%",
  tech: ["Next.js", "React", "GSAP", "Three.js"],
  philosophy: "Technology should always feel like art"
};
```


## 🌐 medaly.engineer
## Tech: Terminal UI, ASCII Art, SSH Access

An interactive terminal-based portfolio accessible via web and SSH.
Because portfolios don't have to be limited to traditional websites.

- Beautiful ASCII art and terminal aesthetics
- Fully keyboard-navigable interface
- Clean, minimalist design
- Remote SSH access capabilities
- Rust-powered backend for performance

```bash
# Access it remotely
curl https://www.medaly.engineer
```


## 🎓 Academic & Learning Projects

### IT Engineering @ ESPRIT University
Currently pursuing advanced studies in Software Engineering, with focus on:
- Modern web architecture
- Cloud infrastructure and DevOps
- System design and scalability
- Advanced programming concepts

### Mechatronics Engineering Background
Original engineering foundation providing unique perspective on:
- Systems thinking and integration
- Hardware-software interaction
- Problem-solving from multiple angles


## 📱 Content Creation

### Tech & DevOps Content
Creating engaging content about:
- DevOps best practices
- Linux tips and tricks (yes, Arch!)
- Creative coding and web animations
- Behind-the-scenes at KOYOTEC
- Developer life and workflows

Follow along on Instagram for Reels and tech insights!


## 🚀 Upcoming Projects

- Advanced 3D web experiences
- Interactive storytelling platforms
- Open-source contributions
- DevOps automation tools
- Creative experiments with emerging tech


## 💡 Collaboration Opportunities

Interested in working together? I'm open to:
- Freelance web development projects
- Creative collaborations
- Content partnerships
- Tech community initiatives
- Mentorship and knowledge sharing


Want to collaborate? Check out my contact page!
"#.to_string()
    }

    fn about_content() -> String {
        r#"
# About Me

## The Journey from Mechatronics to Digital Innovation


  ┌─────────────────────────────────────────────────────────────┐
  │                                                             │
  │   "I believe technology should always feel like art."       │
  │                    - Med Ali Dridi                          │
  │                                                             │
  └─────────────────────────────────────────────────────────────┘


## My Journey

Hi! I'm Med Ali Dridi, and my path into tech has been anything but 
conventional. I started as a Mechatronics Engineer, fascinated by how 
hardware and software come together to create intelligent systems.

But somewhere along the way, I fell in love with the web - not just 
building websites, but creating experiences that feel alive, that tell 
stories, that push the boundaries of what's possible in a browser.

So I made the leap into Software Engineering and DevOps, and I haven't 
looked back since.


## What Drives Me

-  Creating web experiences that feel like art
-  Pushing the limits of web performance and animation
-  Building robust, scalable infrastructure
-  Innovation at the intersection of design and technology
-  Empowering businesses through digital transformation
-  Continuous learning and experimentation
-  Sharing knowledge with the community


## Education

### IT Engineering Student
ESPRIT University | 2024 - 2027
- Specializing in Software Engineering & DevOps
- Focus on modern web technologies and cloud infrastructure
- Building next-generation web applications
- Active in tech communities and student organizations

### Mechatronics Engineering
Graduated | Previous Degree
- Foundation in systems engineering
- Hardware-software integration
- Problem-solving and analytical thinking
- Unique perspective on technology development


## Professional Experience

### Founder & CEO | KOYOTEC DIGITAL
2024 - Present | Tunisia
- Founded and leading a top 1% creative digital agency
- Building next-gen websites with cutting-edge technologies
- Managing client relationships and project delivery
- Creating innovative solutions for local and international clients
- Leading a team of creative developers and designers

### Content Creator
Ongoing
- Creating engaging tech content on Instagram and other platforms
- Sharing DevOps tips, web development insights, and creative process
- Building a community around technology and creativity
- Making tech education fun and accessible


## Technical Skills

### Frontend Development
Next.js, React, JavaScript/TypeScript, GSAP (animations),
Three.js (3D graphics), HTML5, CSS3, Modern Web APIs

### DevOps & Infrastructure
Linux (Arch user!), Docker, Azure Kubernetes Service (AKS),
CI/CD pipelines, Containerization, Cloud Infrastructure

### Other Languages & Tools
Rust, Git, Terminal/Shell, SSH, Web Performance Optimization

### Soft Skills
Creative problem-solving, Project management, Client relations,
Content creation, Team leadership, Technical communication


## KOYOTEC DIGITAL

Our agency is part of the top 1% most creative agencies in Tunisia.
We specialize in:
- High-performance web applications
- Stunning animations and micro-interactions
- 3D web experiences with Three.js
- Modern, responsive designs
- Creative storytelling through technology

We work with clients who aren't afraid to be different, who want 
their digital presence to stand out and make an impact.


## Beyond Code

When I'm not coding or creating content, you can find me:
-  Debugging at 1 a.m. (my most productive hours!)
-  Creating Reels about tech and creativity
-  Exploring new web technologies and frameworks
-  Experimenting with 3D and animation techniques
-  Contributing to tech communities
-  Learning about design and storytelling
-  Finding inspiration in unexpected places


## Philosophy

Technology is more than just code and servers. It's a medium for 
creativity, a canvas for innovation, and a tool for storytelling.

I believe that:
- Every website should be an experience, not just a collection of pages
- Performance and beauty aren't mutually exclusive
- The best solutions come from thinking outside the box
- Technology should inspire, not intimidate
- Code can be art, and art can be code
- Documentation is a love letter to future developers
- The best way to learn is by building and sharing


## Fun Facts

-  I use Arch Linux (btw)
-  Every project is a chance to push creative boundaries
-  Based in Tunisia, working with clients worldwide
-  Active in tech communities and open to mentorship
-  Always experimenting with emerging technologies
-  Goal: Make the web more beautiful, one project at a time


## Community & Impact

I'm passionate about giving back to the tech community:
- Creating educational content for aspiring developers
- Sharing insights about DevOps and modern web development
- Supporting the local tech ecosystem in Tunisia
- Open to mentorship and collaboration opportunities


Let's build something extraordinary together!
"#.to_string()
    }

    fn contact_content() -> String {
        r#"
# Get In Touch

## Let's Create Something Amazing Together


  ╔══════════════════════════════════════════════════════════════╗
  ║                                                              ║
  ║   I'm always excited to discuss new projects, creative       ║
  ║   ideas, or opportunities to collaborate.                    ║
  ║                                                              ║
  ╚══════════════════════════════════════════════════════════════╝


##  Social Media & Links

### Instagram
https://instagram.com/medaly.dridi
Follow me for tech content, Reels, and behind-the-scenes!

### GitHub
https://github.com/mohamedalidridii
Check out my open-source work and projects

### LinkedIn
https://linkedin.com/in/med-ali-dridi
Let's connect professionally

### Linktree
https://linktr.ee/medaly.dridi
All my links in one place


##  Ways to Connect

### For Business Inquiries
- Web development projects (KOYOTEC DIGITAL)
- Creative collaborations and partnerships
- Agency services and consultations
- Technical solutions for your business

### For Development Projects
- Next.js & React applications
- High-performance web experiences
- 3D and animated websites
- DevOps and infrastructure consulting

### For Content & Community
- Collaboration on tech content
- Guest posts and features
- Community initiatives
- Knowledge sharing and mentorship


##  Location & Availability

### Based In
Tunisia (Open to remote work worldwide)

### KOYOTEC DIGITAL
Top 1% Creative Digital Agency in Tunisia
Serving local and international clients

### Availability
- Agency services: Available for new projects
- Freelance: Selective projects that inspire
- Content collaboration: Always open to interesting ideas
- Mentorship: Happy to help aspiring developers


##  What I Can Help With

- Modern web development (Next.js, React)
- Stunning animations and 3D experiences (GSAP, Three.js)
- DevOps and infrastructure (Docker, Kubernetes, CI/CD)
- Website performance optimization
- Creative digital solutions
- Content creation and technical writing
- Career advice for transitioning engineers


##  KOYOTEC DIGITAL Services

- Custom web application development
- High-performance, animated websites
- 3D interactive experiences
- E-commerce solutions
- Brand identity and digital presence
- Technical consulting and architecture
- Ongoing maintenance and support


##  Before You Reach Out

To help me respond effectively:
- Brief introduction about yourself or your company
- What you're looking to build or achieve
- Timeline and budget range (if applicable)
- Any specific requirements or constraints
- Where you found me / what interested you


##  Current Status

```
Agency: KOYOTEC DIGITAL - Active & Taking Projects
Personal: Open to Interesting Collaborations
Student: IT Engineering @ ESPRIT (2024-2027)
Location: Tunisia
Remote Work: ✓ Available Worldwide
```


##  Let's Collaborate!

Whether you have:
- An ambitious web project
- A creative idea that needs technical expertise
- A business looking for digital transformation
- Questions about tech, DevOps, or career transitions
- Content collaboration opportunities

I'd love to hear from you!


  ┌───────────────────────────────────────────────────────────┐
  │                                                           │
  │   "Technology should always feel like art."              │
  │                    Let's make it happen.                 │
  │                                                           │
  └───────────────────────────────────────────────────────────┘


##  Terminal Access

Want to access this portfolio remotely?

```bash
# Get this page
curl https://www.medaly.engineer

# Coming soon: SSH access
# ssh portfolio@medaly.engineer -p 2222
```


##  Quick Response

I typically respond to:
- Business inquiries: Within 24-48 hours
- Collaboration requests: Within 48 hours
- General questions: When time allows
- Community engagement: Always happy to connect!


Looking forward to building something amazing with you! ✨

P.S. Yes, I use Arch btw 🐧
"#.to_string()
    }
}
