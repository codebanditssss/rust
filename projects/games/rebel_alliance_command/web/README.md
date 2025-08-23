# 🌟 Star Wars: Rebel Command - TypeScript Frontend

## 🚀 Setup Instructions

### Prerequisites
- Node.js 18+ and npm
- Rust backend server running on port 8080

### Installation

1. **Navigate to web directory:**
```bash
cd rust_day2/rebel_alliance_command/web
```

2. **Install dependencies:**
```bash
npm install
```

3. **Start development server:**
```bash
npm run dev
```

4. **Open browser:**
```
http://localhost:3000
```

## 🛠️ Build for Production

```bash
npm run build
```

This creates a `dist/` folder that the Rust server can serve.

## 🎮 How It Works

### Frontend-Backend Communication

1. **Frontend (TypeScript/React)** runs on port 3000
2. **Backend (Rust)** runs on port 8080
3. **API calls** go through Vite proxy: `/api` → `http://localhost:8080/api`

### API Endpoints

- `POST /api/game/create` - Start new game
- `GET /api/game/:id` - Get current state
- `POST /api/game/:id/choice` - Make decision

### TypeScript Types

All types match the Rust API structures:
- `GameState` - Complete game data
- `GameOption` - Available choices
- `ApiResponse<T>` - Wrapper for responses

## 🎨 Features

### ✨ Interactive UI
- **Real-time status updates**
- **Beautiful Star Wars theme**
- **Animated backgrounds and effects**
- **Responsive design** (works on mobile)

### 🎯 Game Mechanics
- **Strategic decision making**
- **Resource management** (reputation, credits, Force points)
- **Multiple victory paths**
- **Consequence-based gameplay**

### 🌐 Modern Web Tech
- **TypeScript** for type safety
- **React** for UI components
- **Vite** for fast development
- **Axios** for API communication

## 📁 Project Structure

```
web/
├── src/
│   ├── components/       # React components
│   │   ├── CommanderInput.tsx
│   │   ├── GameInterface.tsx
│   │   ├── StatusPanel.tsx
│   │   ├── ChoiceButton.tsx
│   │   ├── GameOverScreen.tsx
│   │   └── LoadingScreen.tsx
│   ├── types.ts         # TypeScript interfaces
│   ├── api.ts          # API client
│   ├── App.tsx         # Main app component
│   ├── App.css         # App-specific styles
│   ├── index.css       # Global Star Wars theme
│   └── main.tsx        # Entry point
├── package.json        # Dependencies
├── tsconfig.json       # TypeScript config
├── vite.config.ts      # Vite config
└── index.html          # HTML template
```

## 🔧 Development

### Running Both Servers

**Terminal 1 (Rust backend):**
```bash
cd rust_day2/rebel_alliance_command
cargo run
# Choose option 2 for web server
```

**Terminal 2 (TypeScript frontend):**
```bash
cd rust_day2/rebel_alliance_command/web
npm run dev
```

### API Testing

Test the backend directly:
```bash
# Create game
curl -X POST http://localhost:8080/api/game/create \
  -H "Content-Type: application/json" \
  -d '{"commander_name": "Luke"}'

# Make choice
curl -X POST http://localhost:8080/api/game/current/choice \
  -H "Content-Type: application/json" \
  -d '{"choice": 1}'
```

## 🎨 Customization

### Styling
- Edit `src/index.css` for global theme
- Modify component-specific styles in each `.tsx` file
- Use CSS custom properties for consistent theming

### Game Logic
- The frontend is purely UI - all game logic is in Rust
- Add new API endpoints in `src/api.rs` (Rust)
- Create corresponding TypeScript types in `src/types.ts`

## 🚀 Deployment

### Production Build
```bash
npm run build
```

### Serve from Rust
The Rust server automatically serves the built frontend from `web/dist/`

### External Hosting
Deploy `dist/` folder to:
- Vercel
- Netlify
- GitHub Pages
- Any static hosting service

Update API base URL for external hosting.

## 🎯 Game Flow

1. **Enter commander name** → Creates new game
2. **Phase 1: Rescue Leia** → Strategic choices with consequences
3. **Phase 2: Decode Plans** → Resource management decisions  
4. **Phase 3: Prepare Battle** → Fleet and pilot management
5. **Phase 4: Death Star Assault** → Multiple victory paths
6. **Game Over** → Different endings based on choices

## 💡 Tips for Players

- **Build reputation early** - unlocks better options
- **Manage resources wisely** - credits and Force points are limited
- **Read requirements carefully** - some choices need specific stats
- **Multiple playthroughs** - try different strategies for different endings

---

**May the Force be with you! ✨**
