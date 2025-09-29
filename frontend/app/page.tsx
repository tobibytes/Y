"use client";
import React, { useState, useEffect } from 'react';
import { Globe, Users, Zap, ChevronRight} from 'lucide-react';
import Image from 'next/image';
import { Button } from '@/components/ui/button';
import { useRouter } from 'next/navigation';
const YLandingPage = () => {
  const router = useRouter();
  const [currentSlide, setCurrentSlide] = useState(0);
  const [isVisible, setIsVisible] = useState(false);

  const mockPosts = [
    { username: "maya", handle: "@maya", content: "Just joined Y 🌟 The UI is so clean, feels like home already!", timestamp: "2m ago" },
    { username: "jaden", handle: "@jaden", content: "Loving the conversations here. Way less noise than other platforms 👀", timestamp: "10m ago" },
    { username: "ike", handle: "@ike", content: "Day 1 on Y and I already found a crew of amazing people 🔥", timestamp: "1h ago" }
  ];

  const features = [
    { icon: Globe, title: "Stay connected", description: "Discover what's happening across the world and in your community." },
    { icon: Users, title: "Build your community", description: "Follow friends, creators, and voices that matter to you." },
    { icon: Zap, title: "Simple & fast", description: "No clutter, no noise. Just conversations that flow." }
  ];

  useEffect(() => {
    setIsVisible(true);
    const interval = setInterval(() => {
      setCurrentSlide((prev) => (prev + 1) % 3);
    }, 4000);
    return () => clearInterval(interval);
  }, []);

  const MockScreenshot = ({ isActive, index }) => (
    <div className={`absolute inset-0 transition-all duration-1000 ease-out ${
      isActive ? 'opacity-100 translate-x-0' : 'opacity-0 translate-x-12'
    }`}>
      <div className="bg-gradient-to-br from-slate-50 to-slate-100 rounded-2xl p-6 h-full shadow-2xl border border-blue-100">
        <div className="bg-gradient-to-r from-blue-500 to-blue-600 rounded-xl h-12 mb-4 flex items-center px-4">
          <div className="text-white font-bold text-lg">Y</div>
          <div className="ml-auto text-white text-sm opacity-75">Feed {index + 1}</div>
        </div>
        <div className="space-y-4">
          {mockPosts.slice(index, index + 3).map((post, i) => (
            <div key={i} className="bg-white rounded-xl p-4 shadow-md border border-slate-200 transform hover:scale-105 transition-transform duration-300">
              <div className="flex items-start space-x-3">
                <div className={`w-10 h-10 rounded-full bg-gradient-to-br ${
                  i === 0 ? 'from-pink-400 to-purple-500' : 
                  i === 1 ? 'from-blue-400 to-indigo-500' : 
                  'from-green-400 to-teal-500'
                }`}></div>
                <div className="flex-1">
                  <div className="flex items-center space-x-2">
                    <span className="font-semibold text-gray-900">{post.username}</span>
                    <span className="text-gray-500 text-sm">{post.handle}</span>
                    <span className="text-gray-400 text-xs">{post.timestamp}</span>
                  </div>
                  <p className="text-gray-800 mt-1">{post.content}</p>
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );

  return (
    <div className="min-h-screen bg-white font-sans">
      {/* Header */}
      <header className="sticky top-0 bg-white/95 backdrop-blur-md border-b border-blue-100 z-50">
        <div className="max-w-7xl mx-auto px-6 py-4">
          <nav className="flex items-center justify-between">
            <a href="#" className="text-5xl font-bold text-blue-500 hover:scale-105 transition-transform duration-300">
              Y
            </a>
            <div className="hidden md:flex items-center space-x-8">
              <a href="#" className="text-gray-700 hover:text-blue-500 transition-colors font-medium">About</a>
              <a href="#" className="text-gray-700 hover:text-blue-500 transition-colors font-medium">Explore</a>
              <a href="#" className="px-6 py-2 border-2 border-blue-500 text-blue-500 rounded-full hover:bg-blue-500 hover:text-white transition-all duration-300 font-semibold">
                Login
              </a>
              <a href="#" className="px-6 py-2 bg-gradient-to-r from-blue-500 to-blue-600 text-white rounded-full hover:from-blue-600 hover:to-blue-700 transition-all duration-300 shadow-lg hover:shadow-xl transform hover:-translate-y-1 font-semibold">
                Sign Up
              </a>
            </div>
          </nav>
        </div>
      </header>

      {/* Hero Section */}
      <section className="py-16 px-6">
        <div className="max-w-7xl mx-auto">
          <div className={`grid lg:grid-cols-2 gap-16 items-center transition-all duration-1000 ${
            isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'
          }`}>
            {/* Left - Screenshots */}
            <div className="relative">
              <div className="relative h-96 lg:h-[500px] overflow-hidden rounded-3xl shadow-2xl">
                {[0, 1, 2].map((index) => (
                  <MockScreenshot key={index} isActive={currentSlide === index} index={index} />
                ))}
              </div>
              <div className="flex justify-center mt-6 space-x-2">
                {[0, 1, 2].map((index) => (
                  <button
                    key={index}
                    onClick={() => setCurrentSlide(index)}
                    className={`w-3 h-3 rounded-full transition-all duration-300 ${
                      currentSlide === index ? 'bg-blue-500 scale-125' : 'bg-gray-300'
                    }`}
                  />
                ))}
              </div>
            </div>

            {/* Right - Content */}
            <div className="space-y-8">
              <div>
                <h1 className="text-5xl lg:text-6xl font-bold leading-tight mb-4">
                  <span className="bg-gradient-to-r from-gray-900 to-blue-600 bg-clip-text text-transparent">
                    See what's happening right now.
                  </span>
                </h1>
                <p className="text-xl text-gray-600 mb-8">
                  Join Y and start sharing your world in a cleaner, smarter space.
                </p>
              </div>

              {/* Signup Form */}
              <div>
                
                <Button className='h-12 rounded-full p-2' onClick={()=>{router.push('/auth/callback')}}>
                  <Image src={'/Google__G__logo.svg'} alt="Google Logo" width={36} height={36}/> Continue with Google <ChevronRight className="ml-2"/>
                </Button>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="py-20 bg-gradient-to-br from-slate-50 to-white">
        <div className="max-w-7xl mx-auto px-6">
          <h2 className="text-4xl lg:text-5xl font-bold text-center mb-16 text-gray-900">
            Why join Y?
          </h2>
          <div className="grid md:grid-cols-3 gap-8">
            {features.map((feature, index) => {
              const Icon = feature.icon;
              return (
                <div
                  key={index}
                  className="text-center p-8 bg-white rounded-3xl shadow-lg hover:shadow-2xl transition-all duration-500 transform hover:-translate-y-4 group border border-gray-100"
                >
                  <div className="inline-flex items-center justify-center w-20 h-20 bg-gradient-to-br from-blue-500 to-blue-600 rounded-2xl mb-6 group-hover:scale-110 transition-transform duration-300">
                    <Icon className="w-10 h-10 text-white" />
                  </div>
                  <h3 className="text-2xl font-bold mb-4 text-gray-900">{feature.title}</h3>
                  <p className="text-gray-600 text-lg leading-relaxed">{feature.description}</p>
                </div>
              );
            })}
          </div>
        </div>
      </section>

      {/* Feed Preview Section */}
      <section className="py-20 bg-white">
        <div className="max-w-4xl mx-auto px-6">
          <h2 className="text-4xl lg:text-5xl font-bold text-center mb-16 text-gray-900">
            What people are saying
          </h2>
          <div className="space-y-6">
            {mockPosts.map((post, index) => (
              <div
                key={index}
                className="bg-gradient-to-r from-white to-slate-50 p-6 rounded-3xl shadow-lg hover:shadow-xl transition-all duration-300 border border-gray-100 group hover:scale-105"
              >
                <div className="flex items-start space-x-4">
                  <div className={`w-12 h-12 rounded-full bg-gradient-to-br ${
                    index === 0 ? 'from-pink-400 to-purple-500' : 
                    index === 1 ? 'from-blue-400 to-indigo-500' : 
                    'from-green-400 to-teal-500'
                  }`}></div>
                  <div className="flex-1">
                    <div className="flex items-center space-x-3 mb-2">
                      <span className="font-bold text-gray-900">{post.username}</span>
                      <span className="text-gray-500">{post.handle}</span>
                      <span className="text-gray-400 text-sm">{post.timestamp}</span>
                    </div>
                    <p className="text-gray-800 text-lg">{post.content}</p>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* Footer */}
      <footer className="bg-gray-900 text-white py-16">
        <div className="max-w-7xl mx-auto px-6">
          <div className="grid md:grid-cols-4 gap-8 mb-8">
            <div>
              <div className="text-4xl font-bold text-blue-400 mb-4">Y</div>
              <p className="text-gray-400">Your voice. Your space.</p>
            </div>
            <div>
              <h4 className="font-semibold mb-4">Company</h4>
              <div className="space-y-2">
                <a href="#" className="block text-gray-400 hover:text-white transition-colors">About</a>
                <a href="#" className="block text-gray-400 hover:text-white transition-colors">Careers</a>
                <a href="#" className="block text-gray-400 hover:text-white transition-colors">Newsroom</a>
              </div>
            </div>
            <div>
              <h4 className="font-semibold mb-4">Resources</h4>
              <div className="space-y-2">
                <a href="#" className="block text-gray-400 hover:text-white transition-colors">Help Center</a>
                <a href="#" className="block text-gray-400 hover:text-white transition-colors">Safety</a>
                <a href="#" className="block text-gray-400 hover:text-white transition-colors">API</a>
              </div>
            </div>
            <div>
              <h4 className="font-semibold mb-4">Legal</h4>
              <div className="space-y-2">
                <a href="#" className="block text-gray-400 hover:text-white transition-colors">Privacy Policy</a>
                <a href="#" className="block text-gray-400 hover:text-white transition-colors">Terms of Service</a>
                <a href="#" className="block text-gray-400 hover:text-white transition-colors">Cookies</a>
              </div>
            </div>
          </div>
          <div className="border-t border-gray-800 pt-8 text-center text-gray-400">
            © 2025 Y. Connect freely.
          </div>
        </div>
      </footer>
    </div>
  );
};

export default YLandingPage;