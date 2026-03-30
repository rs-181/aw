// app.js - Premium JavaScript for Amrutam Water

// ========== PRELOADER ==========
window.addEventListener('load', function() {
    const preloader = document.getElementById('preloader');
    if (preloader) {
        setTimeout(() => {
            preloader.style.opacity = '0';
            setTimeout(() => {
                preloader.style.display = 'none';
            }, 500);
        }, 1000);
    }
});

// ========== WELCOME MODAL ==========
const modal = document.getElementById('welcomeModal');
const closeBtn = document.getElementById('closeWelcomeBtn');
const mainContent = document.getElementById('mainContent');
const whatsappFloat = document.getElementById('whatsappFloat');
const aiChatFloat = document.getElementById('aiChatFloat');

if (closeBtn) {
    closeBtn.addEventListener('click', function() {
        modal.style.opacity = '0';
        setTimeout(() => {
            modal.style.display = 'none';
            mainContent.style.display = 'block';
            if (whatsappFloat) whatsappFloat.style.display = 'flex';
            if (aiChatFloat) aiChatFloat.style.display = 'flex';
            document.body.style.overflow = 'auto';
        }, 300);
    });
}

// ========== SLIDER FUNCTIONALITY ==========
const sliderTrack = document.getElementById('sliderTrack');
const slides = document.querySelectorAll('.slide');
const prevBtn = document.getElementById('prevSlide');
const nextBtn = document.getElementById('nextSlide');
const dotsContainer = document.getElementById('sliderDots');

let currentIndex = 0;
const totalSlides = slides.length;

// Create dots
if (dotsContainer && totalSlides > 0) {
    for (let i = 0; i < totalSlides; i++) {
        const dot = document.createElement('div');
        dot.classList.add('dot');
        if (i === 0) dot.classList.add('active');
        dot.addEventListener('click', () => goToSlide(i));
        dotsContainer.appendChild(dot);
    }
}

function updateSlider() {
    if (sliderTrack) {
        sliderTrack.style.transform = `translateX(-${currentIndex * 100}%)`;
    }
    // Update dots
    const dots = document.querySelectorAll('.dot');
    dots.forEach((dot, index) => {
        if (index === currentIndex) {
            dot.classList.add('active');
        } else {
            dot.classList.remove('active');
        }
    });
}

function goToSlide(index) {
    currentIndex = index;
    updateSlider();
}

function nextSlide() {
    currentIndex = (currentIndex + 1) % totalSlides;
    updateSlider();
}

function prevSlide() {
    currentIndex = (currentIndex - 1 + totalSlides) % totalSlides;
    updateSlider();
}

if (prevBtn) prevBtn.addEventListener('click', prevSlide);
if (nextBtn) nextBtn.addEventListener('click', nextSlide);

// Auto slide every 5 seconds
let autoSlideInterval = setInterval(nextSlide, 5000);

// Pause auto slide on hover
const sliderContainer = document.querySelector('.slider-container');
if (sliderContainer) {
    sliderContainer.addEventListener('mouseenter', () => {
        clearInterval(autoSlideInterval);
    });
    sliderContainer.addEventListener('mouseleave', () => {
        autoSlideInterval = setInterval(nextSlide, 5000);
    });
}

// ========== MOBILE MENU ==========
const mobileMenuBtn = document.querySelector('.mobile-menu-btn');
const navLinks = document.querySelector('.nav-links');

if (mobileMenuBtn) {
    mobileMenuBtn.addEventListener('click', () => {
        navLinks.classList.toggle('active');
    });
}

// ========== CALCULATOR ==========
const calcBtn = document.getElementById('calcBtn');
const weightInput = document.getElementById('weight');
const activitySelect = document.getElementById('activity');
const resultBox = document.getElementById('result');

if (calcBtn) {
    calcBtn.addEventListener('click', function() {
        const weight = parseFloat(weightInput.value);
        const activity = parseFloat(activitySelect.value);
        
        if (isNaN(weight) || weight <= 0) {
            resultBox.innerHTML = '❌ માન્ય વજન દાખલ કરો';
            resultBox.style.color = '#dc2626';
            return;
        }
        
        const waterNeeded = (weight * 0.033 * activity).toFixed(1);
        resultBox.innerHTML = `💧 તમારા માટે દૈનિક ${waterNeeded} લિટર પાણી જરૂરી છે.`;
        resultBox.style.color = '#1e3a8a';
    });
}

// ========== FEEDBACK FORM ==========
const feedbackForm = document.getElementById('feedbackForm');
if (feedbackForm) {
    feedbackForm.addEventListener('submit', function(e) {
        e.preventDefault();
        const name = document.getElementById('name').value;
        const mobile = document.getElementById('mobile').value;
        const message = document.getElementById('message').value;
        
        if (!name || !message) {
            alert('કૃપા કરીને નામ અને સંદેશ ભરો');
            return;
        }
        
        const whatsappText = `નામ: ${name}%0Aમોબાઇલ: ${mobile || 'N/A'}%0Aસંદેશ: ${message}`;
        window.open(`https://wa.me/919537906611?text=${whatsappText}`, '_blank');
        feedbackForm.reset();
        alert('સંદેશ WhatsApp પર મોકલાઈ ગયો!');
    });
}

// ========== AI CHAT FUNCTIONALITY ==========
const aiChatBtn = document.getElementById('aiChatFloat');
const aiChatWindow = document.getElementById('aiChatWindow');
const closeChatBtn = document.getElementById('closeChatBtn');
const chatInput = document.getElementById('chatInput');
const sendChatBtn = document.getElementById('sendChatBtn');
const chatMessages = document.getElementById('chatMessages');

// Intelligent AI Responses
function getAIResponse(question) {
    const q = question.toLowerCase();
    
    if (q.includes('ભાવ') || q.includes('કિંમત') || q.includes('price')) {
        return "📊 **ભાવ સૂચિ:**\n\n**20 લિટર બોટલ:**\n• નિયમિત ગ્રાહક: ₹15 (મફત ડિલિવરી)\n• જથ્થાબંધ: ₹20\n\n**કોલ્ડ જાર:**\n• નિયમિત ગ્રાહક: ₹35 (મફત ડિલિવરી)\n• જથ્થાબંધ: ₹45";
    }
    
    if (q.includes('ઓર્ડર') || q.includes('order')) {
        return "🛒 **ઓર્ડર કરવા માટે:**\n\n1️⃣ WhatsApp બટન દબાવો\n2️⃣ અથવા +91 95379 06611 પર કૉલ કરો\n3️⃣ અથવા ઉત્પાદનની નીચે આપેલ 'WhatsApp પર ઓર્ડર કરો' બટન દબાવો";
    }
    
    if (q.includes('મફત ડિલિવરી') || q.includes('free delivery')) {
        return "🚚 **મફત ડિલિવરી:**\n\n✅ નિયમિત ગ્રાહકો માટે સંપૂર્ણ મફત ડિલિવરી\n❌ જથ્થાબંધ ઓર્ડર પર ડિલિવરી ચાર્જ લાગુ પડે છે";
    }
    
    if (q.includes('સમય') || q.includes('time')) {
        return "⏰ **સેવા સમય:**\n\n• સવારે 6:00 થી સાંજે 4:00\n• રવિવારે પણ સેવા ઉપલબ્ધ";
    }
    
    if (q.includes('આદર') || q.includes('respect')) {
        return "🙏 **સ્ટાફ પ્રત્યેનો આદર:**\n\nઅમારા સ્ટાફ સાથે અપમાનજનક ભાષા, મારામારી અથવા અભદ્ર વર્તન બિલકુલ સહન કરવામાં આવશે નહીં. અપમાન કરનાર ગ્રાહકની સેવા તાત્કાલિક બંધ કરવામાં આવશે.";
    }
    
    if (q.includes('મોરબી') || q.includes('delivery area')) {
        return "📍 **ડિલિવરી એરિયા:**\n\n• મોરબી સિટી (તમામ વોર્ડ)\n• જયભાવની નગર, લાખા પાટી\n• ગુંદાસર રોડ, 80 ફૂટ રોડ\n• ટ્રાન્સપોર્ટ નગર, મકાન નગર\n• નવી મોરબી, ઓલ્ડ મોરબી\n• ઇન્ડસ્ટ્રીયલ એરિયા";
    }
    
    if (q.includes('શુદ્ધ') || q.includes('quality')) {
        return "💧 **પાણીની ગુણવત્તા:**\n\n• RO અને UV શુદ્ધિકરણ\n• BPA-ફ્રી બોટલ\n• ૧૬ વર્ષથી ગુણવત્તામાં કોઈ સમાધાન નથી\n• નિયમિત ગુણવત્તા તપાસ";
    }
    
    if (q.includes('સંપર્ક') || q.includes('contact')) {
        return "📞 **સંપર્ક માહિતી:**\n\n📱 WhatsApp/કૉલ: +91 95379 06611\n⏰ સમય: સવારે 6:00 - સાંજે 4:00\n📍 સરનામું: મોરબી, ગુજરાત";
    }
    
    return "🙏 આપના પ્રશ્ન માટે આભાર. હું અમૃતમ AI સહાયક છું. કૃપા કરીને વધુ માહિતી માટે +91 95379 06611 પર સંપર્ક કરો. અમે તમારી મદદ માટે હંમેશા તૈયાર છીએ.";
}

function addBotMessage(text) {
    const messageDiv = document.createElement('div');
    messageDiv.className = 'bot-message';
    messageDiv.innerHTML = `<i class="fas fa-robot"></i> ${text.replace(/\n/g, '<br>')}`;
    chatMessages.appendChild(messageDiv);
    chatMessages.scrollTop = chatMessages.scrollHeight;
}

function addUserMessage(text) {
    const messageDiv = document.createElement('div');
    messageDiv.className = 'user-message';
    messageDiv.innerHTML = `<i class="fas fa-user"></i> ${text}`;
    chatMessages.appendChild(messageDiv);
    chatMessages.scrollTop = chatMessages.scrollHeight;
}

function sendMessage() {
    const message = chatInput.value.trim();
    if (!message) return;
    
    addUserMessage(message);
    chatInput.value = '';
    
    setTimeout(() => {
        const response = getAIResponse(message);
        addBotMessage(response);
    }, 500);
}

if (aiChatBtn) {
    aiChatBtn.addEventListener('click', () => {
        aiChatWindow.style.display = aiChatWindow.style.display === 'flex' ? 'none' : 'flex';
        if (chatMessages.children.length === 0) {
            addBotMessage("નમસ્તે! 👋 હું અમૃતમ AI સહાયક છું. હું તમને ભાવ, ઓર્ડર, ડિલિવરી, સમય અને અન્ય કોઈપણ માહિતી આપી શકું છું. કૃપા કરીને પૂછો.");
        }
    });
}

if (closeChatBtn) {
    closeChatBtn.addEventListener('click', () => {
        aiChatWindow.style.display = 'none';
    });
}

if (sendChatBtn) {
    sendChatBtn.addEventListener('click', sendMessage);
}

if (chatInput) {
    chatInput.addEventListener('keypress', (e) => {
        if (e.key === 'Enter') {
            sendMessage();
        }
    });
}

// ========== SCROLL ANIMATIONS ==========
// Initialize AOS
AOS.init({
    duration: 800,
    once: true,
    offset: 100
});

// GSAP Scroll Animations
gsap.registerPlugin(ScrollTrigger);

// Animate stats numbers
const stats = document.querySelectorAll('.stat-number');
if (stats.length > 0) {
    stats.forEach(stat => {
        const target = parseInt(stat.innerText);
        let current = 0;
        ScrollTrigger.create({
            trigger: stat,
            start: 'top 80%',
            onEnter: () => {
                const interval = setInterval(() => {
                    if (current >= target) {
                        clearInterval(interval);
                    } else {
                        current++;
                        stat.innerText = current + (target > 100 ? '+' : '');
                    }
                }, 50);
            }
        });
    });
}

// Animate feature cards on scroll
gsap.utils.toArray('.feature-card, .product-card, .info-card').forEach(card => {
    gsap.from(card, {
        scrollTrigger: {
            trigger: card,
            start: 'top 85%',
            toggleActions: 'play none none reverse'
        },
        y: 50,
        opacity: 0,
        duration: 0.6,
        ease: 'power3.out'
    });
});

// ========== SMOOTH SCROLL ==========
document.querySelectorAll('a[href^="#"]').forEach(anchor => {
    anchor.addEventListener('click', function(e) {
        e.preventDefault();
        const target = document.querySelector(this.getAttribute('href'));
        if (target) {
            target.scrollIntoView({
                behavior: 'smooth',
                block: 'start'
            });
            // Close mobile menu if open
            if (navLinks && navLinks.classList.contains('active')) {
                navLinks.classList.remove('active');
            }
        }
    });
});

// ========== ACTIVE NAVIGATION LINK ==========
const sections = document.querySelectorAll('section');
const navItems = document.querySelectorAll('.nav-links a');

window.addEventListener('scroll', () => {
    let current = '';
    sections.forEach(section => {
        const sectionTop = section.offsetTop;
        const sectionHeight = section.clientHeight;
        if (pageYOffset >= sectionTop - 150) {
            current = section.getAttribute('id');
        }
    });
    
    navItems.forEach(item => {
        item.classList.remove('active');
        if (item.getAttribute('href') === `#${current}`) {
            item.classList.add('active');
        }
    });
});

// ========== PREVENT SCROLL DURING MODAL ==========
if (modal) {
    document.body.style.overflow = 'hidden';
}