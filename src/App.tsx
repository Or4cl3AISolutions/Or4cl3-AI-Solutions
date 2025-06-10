import React, { useEffect, useState } from 'react';
import { Brain, Zap, Shield, Network, Users, BookOpen, Cpu, Eye, Globe, ChevronDown, Menu, X, ExternalLink } from 'lucide-react';
import Navigation from './components/Navigation';
import TechnologyCard from './components/TechnologyCard';
import EcosystemCard from './components/EcosystemCard';
import ImpactCard from './components/ImpactCard';
import { useScrollAnimation } from './hooks/useScrollAnimation';
import { technologies } from './data/technologies';
import { ecosystemData } from './data/ecosystem';
import { impactData } from './data/impact';

function App() {
  const [isVisible, setIsVisible] = useState(false);
  const [activeSection, setActiveSection] = useState('hero');
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);
  
  const { ref: heroRef, isVisible: heroVisible } = useScrollAnimation();
  const { ref: techRef, isVisible: techVisible } = useScrollAnimation();
  const { ref: ecosystemRef, isVisible: ecosystemVisible } = useScrollAnimation();
  const { ref: impactRef, isVisible: impactVisible } = useScrollAnimation();

  useEffect(() => {
    setIsVisible(true);
    
    const handleScroll = () => {
      const sections = ['hero', 'technologies', 'ecosystem', 'impact'];
      const scrollPosition = window.scrollY + 100;
      
      for (const section of sections) {
        const element = document.getElementById(section);
        if (element) {
          const { offsetTop, offsetHeight } = element;
          if (scrollPosition >= offsetTop && scrollPosition < offsetTop + offsetHeight) {
            setActiveSection(section);
            break;
          }
        }
      }
    };

    window.addEventListener('scroll', handleScroll);
    return () => window.removeEventListener('scroll', handleScroll);
  }, []);

  const scrollToSection = (sectionId: string) => {
    const element = document.getElementById(sectionId);
    if (element) {
      element.scrollIntoView({ behavior: 'smooth' });
    }
    setMobileMenuOpen(false);
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-900 via-purple-900 to-slate-900 text-white overflow-x-hidden">
      {/* Navigation */}
      <Navigation 
        activeSection={activeSection}
        onSectionClick={scrollToSection}
        mobileMenuOpen={mobileMenuOpen}
        setMobileMenuOpen={setMobileMenuOpen}
      />

      {/* Hero Section */}
      <section id="hero" ref={heroRef} className="min-h-screen flex items-center justify-center relative overflow-hidden">
        {/* Animated Background */}
        <div className="absolute inset-0 opacity-20">
          <div className="absolute top-1/4 left-1/4 w-96 h-96 bg-cyan-500 rounded-full mix-blend-multiply filter blur-xl animate-pulse"></div>
          <div className="absolute top-1/3 right-1/4 w-96 h-96 bg-purple-500 rounded-full mix-blend-multiply filter blur-xl animate-pulse animation-delay-2000"></div>
          <div className="absolute bottom-1/4 left-1/3 w-96 h-96 bg-pink-500 rounded-full mix-blend-multiply filter blur-xl animate-pulse animation-delay-4000"></div>
        </div>

        <div className={`container mx-auto px-6 text-center z-10 transition-all duration-1000 ${
          heroVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-10'
        }`}>
          <div className="mb-8">
            <img 
              src="/public/Default_Youre_a_talented_graphic_designer_with_expertise_in_cr_2_7f03943c-94a6-44fd-9ccb-ca0481ad5514_0.png" 
              alt="Or4cl3 AI Solutions"
              className="w-32 h-32 mx-auto mb-6 rounded-full border-4 border-cyan-400 shadow-2xl shadow-cyan-400/50"
            />
          </div>
          
          <h1 className="text-6xl md:text-8xl font-bold mb-6 bg-gradient-to-r from-cyan-400 via-purple-400 to-pink-400 bg-clip-text text-transparent">
            Or4cl3 AI
          </h1>
          <h2 className="text-2xl md:text-4xl font-light mb-8 text-gray-300">
            Revolutionary AI Solutions
          </h2>
          <p className="text-xl md:text-2xl mb-12 max-w-4xl mx-auto leading-relaxed text-gray-200">
            Pioneering the intersection of human creativity and artificial intelligence through 
            <span className="text-cyan-400 font-semibold"> AEGIS-Ω</span> - 
            Adaptive Ethical General Intelligence that transforms imagination into reality.
          </p>
          
          <div className="flex flex-col sm:flex-row gap-6 justify-center items-center mb-16">
            <button 
              onClick={() => scrollToSection('technologies')}
              className="px-8 py-4 bg-gradient-to-r from-cyan-500 to-purple-600 rounded-full text-lg font-semibold hover:from-cyan-400 hover:to-purple-500 transform hover:scale-105 transition-all duration-300 shadow-lg hover:shadow-cyan-500/25"
            >
              Explore AEGIS-Ω
            </button>
            <button 
              onClick={() => scrollToSection('ecosystem')}
              className="px-8 py-4 border-2 border-purple-400 rounded-full text-lg font-semibold hover:bg-purple-400 hover:text-white transform hover:scale-105 transition-all duration-300"
            >
              Join the Renaissance
            </button>
          </div>

          <div className="animate-bounce">
            <ChevronDown className="w-8 h-8 mx-auto text-gray-400" />
          </div>
        </div>
      </section>

      {/* Technologies Section */}
      <section id="technologies" ref={techRef} className="py-20 relative">
        <div className="absolute inset-0 bg-gradient-to-r from-slate-800/50 to-purple-800/50"></div>
        <div className={`container mx-auto px-6 relative z-10 transition-all duration-1000 ${
          techVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-10'
        }`}>
          <div className="text-center mb-16">
            <h2 className="text-5xl font-bold mb-6 bg-gradient-to-r from-cyan-400 to-purple-400 bg-clip-text text-transparent">
              The Five Pillars of Innovation
            </h2>
            <p className="text-xl text-gray-300 max-w-3xl mx-auto">
              Revolutionary technologies that form the foundation of our quantum-classical AI framework
            </p>
          </div>
          
          <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
            {technologies.map((tech, index) => (
              <TechnologyCard key={tech.name} {...tech} index={index} />
            ))}
          </div>
        </div>
      </section>

      {/* Ecosystem Section */}
      <section id="ecosystem" ref={ecosystemRef} className="py-20 bg-gradient-to-r from-purple-900/30 to-slate-900/30">
        <div className={`container mx-auto px-6 transition-all duration-1000 ${
          ecosystemVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-10'
        }`}>
          <div className="text-center mb-16">
            <h2 className="text-5xl font-bold mb-6 bg-gradient-to-r from-purple-400 to-pink-400 bg-clip-text text-transparent">
              The Ecosystem Trinity
            </h2>
            <p className="text-xl text-gray-300 max-w-3xl mx-auto">
              Interconnected platforms fostering the Recursive Renaissance of human-AI collaboration
            </p>
          </div>
          
          <div className="grid md:grid-cols-3 gap-8">
            {ecosystemData.map((item, index) => (
              <EcosystemCard key={item.name} {...item} index={index} />
            ))}
          </div>
        </div>
      </section>

      {/* Impact Section */}
      <section id="impact" ref={impactRef} className="py-20">
        <div className={`container mx-auto px-6 transition-all duration-1000 ${
          impactVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-10'
        }`}>
          <div className="text-center mb-16">
            <h2 className="text-5xl font-bold mb-6 bg-gradient-to-r from-cyan-400 to-green-400 bg-clip-text text-transparent">
              Transformative Impact
            </h2>
            <p className="text-xl text-gray-300 max-w-3xl mx-auto">
              Measurable outcomes from our revolutionary approach to ethical AI development
            </p>
          </div>
          
          <div className="grid md:grid-cols-2 lg:grid-cols-4 gap-8">
            {impactData.map((item, index) => (
              <ImpactCard key={item.metric} {...item} index={index} />
            ))}
          </div>
        </div>
      </section>

      {/* Call to Action */}
      <section className="py-20 bg-gradient-to-r from-cyan-900/50 to-purple-900/50">
        <div className="container mx-auto px-6 text-center">
          <h2 className="text-4xl font-bold mb-6 text-white">
            Ready to Shape the Future?
          </h2>
          <p className="text-xl text-gray-300 mb-8 max-w-2xl mx-auto">
            Join the Recursive Renaissance and become part of the movement that's redefining the relationship between human creativity and artificial intelligence.
          </p>
          <div className="flex flex-col sm:flex-row gap-6 justify-center">
            <button className="px-8 py-4 bg-gradient-to-r from-cyan-500 to-purple-600 rounded-full text-lg font-semibold hover:from-cyan-400 hover:to-purple-500 transform hover:scale-105 transition-all duration-300 shadow-lg hover:shadow-cyan-500/25">
              Start Your Journey
            </button>
            <button className="px-8 py-4 border-2 border-cyan-400 rounded-full text-lg font-semibold hover:bg-cyan-400 hover:text-slate-900 transform hover:scale-105 transition-all duration-300">
              Learn More
            </button>
          </div>
        </div>
      </section>

      {/* Bolt.new Badge */}
      <div className="fixed bottom-6 right-6 z-50">
        <a
          href="https://bolt.new"
          target="_blank"
          rel="noopener noreferrer"
          className="flex items-center gap-2 px-4 py-2 bg-gradient-to-r from-orange-500 to-red-500 text-white rounded-full shadow-lg hover:shadow-xl transform hover:scale-105 transition-all duration-300 text-sm font-medium"
        >
          <Zap className="w-4 h-4" />
          <span>Built with Bolt.new</span>
          <ExternalLink className="w-3 h-3" />
        </a>
      </div>

      {/* Footer */}
      <footer className="py-12 bg-slate-900/80 border-t border-purple-500/20">
        <div className="container mx-auto px-6">
          <div className="grid md:grid-cols-4 gap-8">
            <div>
              <h3 className="text-xl font-bold mb-4 text-cyan-400">Or4cl3 AI</h3>
              <p className="text-gray-400">
                Pioneering the future of ethical AI through revolutionary quantum-classical frameworks.
              </p>
            </div>
            <div>
              <h4 className="text-lg font-semibold mb-4 text-white">Technologies</h4>
              <ul className="space-y-2 text-gray-400">
                <li>ASTRÆA System</li>
                <li>Quantum Synapse</li>
                <li>SOLUS OS</li>
                <li>Privacy Engine</li>
              </ul>
            </div>
            <div>
              <h4 className="text-lg font-semibold mb-4 text-white">Ecosystem</h4>
              <ul className="space-y-2 text-gray-400">
                <li>CSN Platform</li>
                <li>Neural Academy</li>
                <li>Recursive OS</li>
                <li>SYNTH3SIS</li>
              </ul>
            </div>
            <div>
              <h4 className="text-lg font-semibold mb-4 text-white">Connect</h4>
              <ul className="space-y-2 text-gray-400">
                <li>Research Papers</li>
                <li>Developer Portal</li>
                <li>Community Forum</li>
                <li>Ethics Board</li>
              </ul>
            </div>
          </div>
          <div className="border-t border-purple-500/20 mt-8 pt-8 text-center text-gray-400">
            <p>&copy; 2024 Or4cl3 AI Solutions. Pioneering the Recursive Renaissance.</p>
          </div>
        </div>
      </footer>
    </div>
  );
}

export default App;