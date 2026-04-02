<!DOCTYPE html>
<html lang="hi">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0, user-scalable=no">
    <title>TouchType Pro | Master Touch Typing — Zero to Hero</title>
    <meta name="description" content="Learn touch typing with finger guides, real-time feedback, and step-by-step lessons. Free, offline, no ads.">
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        body {
            font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
            background: radial-gradient(circle at 20% 30%, #0a0c15, #03050a);
            min-height: 100vh;
            color: #f0f3fa;
            overflow-x: hidden;
        }

        /* full screen app container */
        .app {
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            display: flex;
            flex-direction: column;
            overflow-y: auto;
            scroll-behavior: smooth;
        }

        /* custom scroll */
        .app::-webkit-scrollbar { width: 6px; }
        .app::-webkit-scrollbar-track { background: #1a1f2e; }
        .app::-webkit-scrollbar-thumb { background: #f5c542; border-radius: 10px; }

        /* header */
        .header {
            padding: 20px 40px;
            display: flex;
            justify-content: space-between;
            align-items: center;
            flex-wrap: wrap;
            background: rgba(5, 8, 18, 0.7);
            backdrop-filter: blur(20px);
            border-bottom: 1px solid rgba(245, 197, 66, 0.2);
            position: sticky;
            top: 0;
            z-index: 100;
        }
        .logo h1 {
            font-size: 1.7rem;
            background: linear-gradient(135deg, #fff, #f5c542);
            -webkit-background-clip: text;
            background-clip: text;
            color: transparent;
        }
        .logo p {
            font-size: 0.75rem;
            opacity: 0.7;
        }
        .stats {
            display: flex;
            gap: 30px;
        }
        .stat {
            text-align: center;
        }
        .stat .value {
            font-size: 1.8rem;
            font-weight: 700;
            color: #f5c542;
        }

        /* main content */
        .main-content {
            flex: 1;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            padding: 30px 24px;
            max-width: 1400px;
            margin: 0 auto;
            width: 100%;
        }

        /* finger guide panel */
        .finger-guide {
            background: rgba(15, 20, 35, 0.6);
            backdrop-filter: blur(16px);
            border-radius: 48px;
            padding: 20px 28px;
            margin-bottom: 30px;
            width: 100%;
            border: 1px solid rgba(245, 197, 66, 0.25);
            transition: all 0.2s;
        }
        .guide-title {
            font-size: 0.9rem;
            text-transform: uppercase;
            letter-spacing: 2px;
            color: #f5c542;
            margin-bottom: 12px;
        }
        .hand-diagram {
            display: flex;
            justify-content: space-between;
            flex-wrap: wrap;
            gap: 20px;
        }
        .hand {
            flex: 1;
            min-width: 240px;
        }
        .hand h4 {
            margin-bottom: 12px;
            font-size: 1rem;
        }
        .finger-keys {
            display: flex;
            gap: 8px;
            flex-wrap: wrap;
        }
        .finger-key {
            background: #1e293b;
            padding: 8px 14px;
            border-radius: 40px;
            font-family: monospace;
            font-weight: bold;
            font-size: 1.1rem;
            border-left: 3px solid #f5c542;
        }
        .finger-label {
            font-size: 0.7rem;
            margin-left: 6px;
            color: #f5c542;
        }

        /* typing arena — no input box! */
        .typing-arena {
            width: 100%;
            background: rgba(0, 0, 0, 0.45);
            border-radius: 56px;
            padding: 32px 28px;
            margin: 20px 0;
            backdrop-filter: blur(8px);
            border: 1px solid rgba(255,255,255,0.08);
        }
        .lesson-header {
            display: flex;
            justify-content: space-between;
            margin-bottom: 28px;
            flex-wrap: wrap;
        }
        .lesson-badge {
            background: #f5c54220;
            padding: 6px 18px;
            border-radius: 60px;
            font-size: 0.85rem;
            border: 1px solid #f5c54240;
        }
        .progress-bar {
            width: 200px;
            height: 6px;
            background: #2d3748;
            border-radius: 10px;
            overflow: hidden;
        }
        .progress-fill {
            height: 100%;
            width: 0%;
            background: #f5c542;
            transition: width 0.1s ease;
        }
        .target-text {
            font-family: 'Fira Mono', 'Courier New', monospace;
            font-size: 2.2rem;
            line-height: 1.4;
            letter-spacing: 1px;
            word-break: break-word;
            background: #010614;
            padding: 28px 24px;
            border-radius: 40px;
            margin-bottom: 20px;
            border: 1px solid #2a3958;
        }
        .current-highlight {
            background: #f5c542;
            color: #0a0c15;
            border-radius: 12px;
            padding: 0 6px;
            display: inline-block;
            animation: subtlePulse 1s infinite;
        }
        .mistake-highlight {
            background: #e53e3e;
            color: white;
            border-radius: 8px;
            text-decoration: underline;
        }
        .typed-correct {
            color: #68d391;
        }
        .live-feedback {
            font-size: 1rem;
            text-align: center;
            margin-top: 16px;
            color: #b9c3db;
        }
        .next-hint {
            font-size: 0.85rem;
            background: #1e2a3e;
            display: inline-block;
            padding: 6px 18px;
            border-radius: 60px;
        }

        /* visual keyboard */
        .visual-keyboard {
            background: rgba(10, 14, 23, 0.8);
            border-radius: 32px;
            padding: 20px;
            margin-top: 20px;
            overflow-x: auto;
        }
        .kb-row {
            display: flex;
            justify-content: center;
            gap: 6px;
            margin-bottom: 8px;
            flex-wrap: nowrap;
        }
        .kb-key {
            background: #111827;
            border-radius: 12px;
            padding: 12px 10px;
            min-width: 55px;
            text-align: center;
            font-weight: 600;
            font-family: monospace;
            transition: 0.07s linear;
            box-shadow: 0 2px 0 #00000055;
            border-bottom: 1px solid #2d3a5e;
        }
        .kb-key.active-finger {
            background: #f5c542;
            color: #000;
            box-shadow: 0 0 12px #f5c542aa;
            transform: scale(1.02);
        }
        .kb-key.pressed {
            transform: scale(0.94);
            background: #e6b422;
        }

        /* level selector */
        .level-strip {
            display: flex;
            gap: 12px;
            flex-wrap: wrap;
            justify-content: center;
            margin: 28px 0 15px;
        }
        .level-dot {
            background: #1f2a44;
            border-radius: 60px;
            padding: 8px 20px;
            font-size: 0.8rem;
            cursor: pointer;
            transition: 0.1s;
        }
        .level-dot.active {
            background: #f5c542;
            color: #0a0c15;
            font-weight: bold;
        }
        .level-dot.completed {
            background: #2c7840;
        }

        button {
            background: #f5c542;
            border: none;
            padding: 12px 28px;
            border-radius: 44px;
            font-weight: bold;
            cursor: pointer;
            color: #0a0c15;
            margin: 8px;
        }
        .certificate-card {
            background: linear-gradient(145deg, #1a1f2e, #0f1320);
            border-radius: 48px;
            padding: 32px;
            text-align: center;
            margin-top: 30px;
        }

        @keyframes subtlePulse {
            0% { opacity: 0.9; transform: scale(1);}
            100% { opacity: 1; transform: scale(1.02);}
        }

        @media (max-width: 800px) {
            .target-text { font-size: 1.4rem; }
            .kb-key { min-width: 38px; padding: 8px 4px; font-size: 12px; }
            .header { padding: 12px 20px; }
            .stat .value { font-size: 1.2rem; }
        }
    </style>
</head>
<body>
<div class="app">
    <div class="header">
        <div class="logo">
            <h1>⌨️ TouchType Pro</h1>
            <p>zero to hero • muscle memory mastery</p>
        </div>
        <div class="stats">
            <div class="stat"><div class="value" id="wpmVal">0</div><div>WPM</div></div>
            <div class="stat"><div class="value" id="accVal">100</div><div>%</div></div>
            <div class="stat"><div class="value" id="mistVal">0</div><div>mistakes</div></div>
        </div>
    </div>

    <div class="main-content">
        <!-- Finger guide section -->
        <div class="finger-guide" id="fingerGuide">
            <div class="guide-title">🎯 FINGER MAPPING — इस level में ये fingers use करो</div>
            <div class="hand-diagram" id="handDiagram"></div>
        </div>

        <!-- Typing arena (no input box) -->
        <div class="typing-arena">
            <div class="lesson-header">
                <div class="lesson-badge" id="lessonName">Home Row</div>
                <div class="progress-bar"><div class="progress-fill" id="progressFill"></div></div>
            </div>
            <div class="target-text" id="targetTextDisplay"></div>
            <div class="live-feedback">
                <span id="feedbackMsg">👉 अब टाइप करना शुरू करें — बस keyboard press करें</span>
                <div class="next-hint" id="nextKeyHint"></div>
            </div>
        </div>

        <!-- Visual Keyboard -->
        <div class="visual-keyboard" id="visualKeyboard"></div>

        <!-- Level Navigation -->
        <div class="level-strip" id="levelStrip"></div>
        <div style="display: flex; gap: 14px; justify-content: center;">
            <button id="resetLessonBtn">⟳ Reset Lesson</button>
            <button id="nextLevelBtn">➡ Next Level</button>
        </div>

        <div id="certificateSection" style="display: none;" class="certificate-card">
            <h2>🏆 TOUCH TYPING HERO 🏆</h2>
            <canvas id="certCanvas" width="500" height="300" style="max-width:100%; background:#fff; border-radius: 24px; margin:16px 0;"></canvas><br>
            <button id="downloadCert">Download Certificate (PNG)</button>
        </div>
    </div>
</div>

<script>
    // ---------- ADVANCED LESSONS (finger + key focus) ----------
    const LESSONS = [
        { name: "🏠 HOME ROW", text: "asdf jkl; asdf jkl; sad lad fall jak flak ask", focusKeys: ["a","s","d","f","j","k","l",";"], fingerMap: { left: { pinky:"a", ring:"s", middle:"d", index:"f" }, right: { index:"j", middle:"k", ring:"l", pinky:";" } } },
        { name: "⬆️ TOP ROW", text: "qwer uiop qwerty uiop type quite word", focusKeys: ["q","w","e","r","u","i","o","p"], fingerMap: { left: { pinky:"q", ring:"w", middle:"e", index:"r" }, right: { index:"u", middle:"i", ring:"o", pinky:"p" } } },
        { name: "⬇️ BOTTOM ROW", text: "zxcvbnm zxcv bnm cxz vbn zxcvbnm", focusKeys: ["z","x","c","v","b","n","m"], fingerMap: { left: { pinky:"z", ring:"x", middle:"c", index:"v" }, right: { index:"b", middle:"n", ring:"m" } } },
        { name: "🔠 SHIFT + CAPITALS", text: "The Quick Brown Fox Jumps Over Lazy Dog. Typing Pro!", focusKeys: ["Shift"], fingerMap: { special: "Shift with pinky" } },
        { name: "🔢 NUMBERS", text: "1234 5678 90 1234567890 5050 9876", focusKeys: ["1","2","3","4","5","6","7","8","9","0"], fingerMap: { left: { pinky:"1", ring:"2", middle:"3", index:"4/5" }, right:{ index:"6/7", middle:"8", ring:"9", pinky:"0" } } },
        { name: "✨ SYMBOLS", text: "!@#$ %^&* ()_+ {} :<>? ~` ;:,.", focusKeys: ["!","@","#","$","%","^","&","*","(",")","_","+"], fingerMap: { note: "Shift + number row" } },
        { name: "⚡ SPEED BUILDER", text: "mastering touch typing builds muscle memory. practice daily for speed and accuracy without seeing keys. focus on rhythm.", focusKeys: ["all"], fingerMap: { all: "all fingers" } },
        { name: "🏁 FINAL HERO", text: "Congratulations hero! You type without looking. Keep practicing and you'll reach 100+ WPM. Never give up!", focusKeys: ["all"], fingerMap: { all: "mastery" } }
    ];

    let currentLessonIdx = 0;
    let currentText = "";
    let currentPos = 0;
    let mistakes = 0;
    let totalKeystrokes = 0;
    let startTime = null;
    let completedLessons = new Array(LESSONS.length).fill(false);
    let lessonFinishedFlag = false;

    // DOM
    const targetDiv = document.getElementById("targetTextDisplay");
    const lessonNameSpan = document.getElementById("lessonName");
    const wpmSpan = document.getElementById("wpmVal");
    const accSpan = document.getElementById("accVal");
    const mistSpan = document.getElementById("mistVal");
    const progressFill = document.getElementById("progressFill");
    const feedbackSpan = document.getElementById("feedbackMsg");
    const nextHintSpan = document.getElementById("nextKeyHint");
    const visualKbDiv = document.getElementById("visualKeyboard");
    const handDiagramDiv = document.getElementById("handDiagram");
    const levelStripDiv = document.getElementById("levelStrip");

    function initLesson() {
        if (currentLessonIdx >= LESSONS.length) {
            showCertificate();
            return;
        }
        lessonFinishedFlag = false;
        currentText = LESSONS[currentLessonIdx].text;
        currentPos = 0;
        mistakes = 0;
        totalKeystrokes = 0;
        startTime = null;
        updateDisplay();
        updateStats();
        updateFingerGuide();
        drawKeyboard();
        updateLevelStrip();
        saveProgress();
        if (currentPos === currentText.length) autoCompleteCheck();
        feedbackSpan.innerText = "✨ Start typing — just press the correct key!";
        nextHintSpan.innerText = `👉 Next: "${currentText[0] || ''}"`;
    }

    function updateDisplay() {
        if (!currentText) return;
        let html = "";
        for (let i=0; i<currentText.length; i++) {
            let ch = currentText[i];
            if (i < currentPos) {
                html += `<span class="typed-correct">${escapeHtml(ch)}</span>`;
            } else if (i === currentPos) {
                html += `<span class="current-highlight">${escapeHtml(ch)}</span>`;
            } else {
                html += escapeHtml(ch);
            }
        }
        targetDiv.innerHTML = html;
        lessonNameSpan.innerText = LESSONS[currentLessonIdx].name;
        let percent = (currentPos / currentText.length) * 100;
        progressFill.style.width = percent + "%";
        if (currentPos < currentText.length) nextHintSpan.innerText = `👉 Next key: "${currentText[currentPos]}"`;
        else nextHintSpan.innerText = "✅ Lesson complete! Press Next Level →";
    }

    function updateStats() {
        let wpm = 0;
        let accuracy = 100;
        if (startTime && totalKeystrokes > 0) {
            let minutes = (Date.now() - startTime) / 60000;
            let words = totalKeystrokes / 5;
            wpm = Math.floor(words / minutes);
            if (wpm < 0) wpm = 0;
        }
        let totalAttempts = currentPos + mistakes;
        if (totalAttempts > 0) accuracy = Math.floor((currentPos / totalAttempts) * 100);
        wpmSpan.innerText = wpm;
        accSpan.innerText = accuracy;
        mistSpan.innerText = mistakes;
    }

    function handleKeyPress(e) {
        if (lessonFinishedFlag) return;
        const pressed = e.key;
        if (pressed === "Shift" || pressed === "Control" || pressed === "Alt" || pressed === "Meta") return;
        e.preventDefault();

        if (!startTime && currentPos === 0) startTime = Date.now();

        const expected = currentText[currentPos];
        if (!expected) return;

        // visual feedback on keyboard
        highlightKeyOnBoard(pressed);

        if (pressed === expected) {
            // correct
            currentPos++;
            totalKeystrokes++;
            updateDisplay();
            updateStats();
            if (currentPos === currentText.length) {
                lessonFinishedFlag = true;
                completedLessons[currentLessonIdx] = true;
                saveProgress();
                feedbackSpan.innerText = "🎉 Level complete! Click 'Next Level' to continue.";
                if (currentLessonIdx + 1 >= LESSONS.length) showCertificate();
            } else {
                feedbackSpan.innerText = "✅ Correct! Keep going.";
            }
        } else {
            // mistake
            mistakes++;
            totalKeystrokes++;
            updateStats();
            updateDisplay();
            feedbackSpan.innerText = `❌ Mistake! Expected "${expected}" but you pressed "${pressed}". Try again.`;
            // visual error flash
            targetDiv.style.transform = "scale(1.01)";
            setTimeout(() => targetDiv.style.transform = "", 120);
        }
    }

    function highlightKeyOnBoard(key) {
        const allKeys = document.querySelectorAll(".kb-key");
        for (let k of allKeys) {
            if (k.innerText === key || (key === " " && k.innerText === "Space")) {
                k.classList.add("pressed");
                setTimeout(() => k.classList.remove("pressed"), 100);
            }
        }
    }

    function drawKeyboard() {
        const rows = [
            ["`","1","2","3","4","5","6","7","8","9","0","-","="],
            ["Tab","q","w","e","r","t","y","u","i","o","p","[","]","\\"],
            ["Caps","a","s","d","f","g","h","j","k","l",";","'","Enter"],
            ["Shift","z","x","c","v","b","n","m",",",".","/","Shift"]
        ];
        const focusSet = new Set(LESSONS[currentLessonIdx].focusKeys);
        visualKbDiv.innerHTML = "";
        rows.forEach(row => {
            let rowDiv = document.createElement("div");
            rowDiv.className = "kb-row";
            row.forEach(key => {
                let keyDiv = document.createElement("div");
                keyDiv.className = "kb-key";
                keyDiv.innerText = key.length > 3 ? key.slice(0,2) : key;
                if (focusSet.has(key.toLowerCase()) || LESSONS[currentLessonIdx].focusKeys[0] === "all") {
                    keyDiv.style.border = "1px solid #f5c542";
                    keyDiv.style.background = "#2a3a55";
                }
                rowDiv.appendChild(keyDiv);
            });
            visualKbDiv.appendChild(rowDiv);
        });
    }

    function updateFingerGuide() {
        const lesson = LESSONS[currentLessonIdx];
        handDiagramDiv.innerHTML = "";
        if (lesson.fingerMap.left || lesson.fingerMap.right) {
            let leftDiv = document.createElement("div");
            leftDiv.className = "hand";
            leftDiv.innerHTML = "<h4>🫲 LEFT HAND</h4><div class='finger-keys'></div>";
            let rightDiv = document.createElement("div");
            rightDiv.className = "hand";
            rightDiv.innerHTML = "<h4>🫱 RIGHT HAND</h4><div class='finger-keys'></div>";
            if (lesson.fingerMap.left) {
                let container = leftDiv.querySelector(".finger-keys");
                for (let [finger, key] of Object.entries(lesson.fingerMap.left)) {
                    container.innerHTML += `<div class="finger-key">${key} <span class="finger-label">(${finger})</span></div>`;
                }
            }
            if (lesson.fingerMap.right) {
                let container = rightDiv.querySelector(".finger-keys");
                for (let [finger, key] of Object.entries(lesson.fingerMap.right)) {
                    container.innerHTML += `<div class="finger-key">${key} <span class="finger-label">(${finger})</span></div>`;
                }
            }
            handDiagramDiv.appendChild(leftDiv);
            handDiagramDiv.appendChild(rightDiv);
        } else {
            handDiagramDiv.innerHTML = `<div style="padding:12px;">🎯 ${lesson.fingerMap.note || lesson.fingerMap.special || "Use proper finger positioning"}</div>`;
        }
    }

    function resetLesson() {
        currentPos = 0;
        mistakes = 0;
        totalKeystrokes = 0;
        startTime = null;
        lessonFinishedFlag = false;
        updateDisplay();
        updateStats();
        feedbackSpan.innerText = "Lesson reset. Start typing!";
    }

    function nextLesson() {
        if (!completedLessons[currentLessonIdx] && currentPos !== currentText.length) {
            feedbackSpan.innerText = "⚠️ Complete current lesson first!";
            return;
        }
        if (currentLessonIdx + 1 < LESSONS.length) {
            currentLessonIdx++;
            initLesson();
        } else {
            showCertificate();
        }
    }

    function showCertificate() {
        document.getElementById("certificateSection").style.display = "block";
        drawCertificateCanvas();
    }

    function drawCertificateCanvas() {
        const canvas = document.getElementById("certCanvas");
        if(!canvas) return;
        canvas.width = 550;
        canvas.height = 350;
        let ctx = canvas.getContext("2d");
        ctx.fillStyle = "#fef7e0";
        ctx.fillRect(0,0,canvas.width,canvas.height);
        ctx.fillStyle = "#b55b1c";
        ctx.font = "bold 26px 'Segoe UI'";
        ctx.fillText("🏆 TOUCH TYPING MASTERY", 100, 70);
        ctx.font = "18px monospace";
        ctx.fillStyle = "#2c3e2f";
        ctx.fillText("This certifies that", 190, 130);
        ctx.font = "bold 22px Inter";
        ctx.fillStyle = "#1f3b4c";
        ctx.fillText("Typing Legend", 190, 180);
        ctx.font = "12px monospace";
        ctx.fillStyle = "#4a5b23";
        ctx.fillText(`Completed all ${LESSONS.length} levels • 0 to Hero • Touch Typing Expert`, 80, 250);
        ctx.fillStyle = "#333";
        ctx.fillText(new Date().toLocaleDateString(), 210, 310);
    }

    function updateLevelStrip() {
        levelStripDiv.innerHTML = "";
        LESSONS.forEach((l, idx) => {
            let btn = document.createElement("div");
            btn.className = "level-dot";
            if (idx === currentLessonIdx) btn.classList.add("active");
            if (completedLessons[idx]) btn.classList.add("completed");
            btn.innerText = `${idx+1}. ${l.name.slice(0,12)}`;
            btn.onclick = () => {
                if (idx <= currentLessonIdx+1 || completedLessons[idx-1]) {
                    currentLessonIdx = idx;
                    initLesson();
                } else alert("Complete previous levels first!");
            };
            levelStripDiv.appendChild(btn);
        });
    }

    function saveProgress() {
        localStorage.setItem("ttpro", JSON.stringify({
            currentLessonIdx, completedLessons
        }));
    }

    function loadProgress() {
        let saved = localStorage.getItem("ttpro");
        if (saved) {
            try {
                let d = JSON.parse(saved);
                currentLessonIdx = d.currentLessonIdx || 0;
                completedLessons = d.completedLessons || new Array(LESSONS.length).fill(false);
            } catch(e) {}
        }
        initLesson();
    }

    function escapeHtml(s) { return s.replace(/[&<>]/g, function(m){ if(m==='&') return '&amp;'; if(m==='<') return '&lt;'; if(m==='>') return '&gt;'; return m;}).replace(/ /g, '&nbsp;'); }

    window.addEventListener("keydown", handleKeyPress);
    document.getElementById("resetLessonBtn").addEventListener("click", resetLesson);
    document.getElementById("nextLevelBtn").addEventListener("click", nextLesson);
    document.getElementById("downloadCert")?.addEventListener("click", function() {
        let canvas = document.getElementById("certCanvas");
        let link = document.createElement("a");
        link.download = "touch_typing_certificate.png";
        link.href = canvas.toDataURL();
        link.click();
    });
    loadProgress();
</script>
</body>
</html>
