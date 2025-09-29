"use client";
import { useState } from "react";
import React from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { Avatar } from "@/components/ui/avatar";
import { Label } from "@/components/ui/label";
import { Calendar } from "@/components/ui/calendar";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { ChevronRight } from "lucide-react";

const OnBoardingPage = () => {
  const [userData, setUserData] = useState<{ name: string; bio: string, birthday: Date, profilePicture: File | null }>({ 
    name: "", 
    bio: "", 
    birthday: new Date(), 
    profilePicture: null 
  });
  const [dropdown, setDropdown] = useState<React.ComponentProps<typeof Calendar>["captionLayout"]>("dropdown");
  const [date, setDate] = useState<Date | undefined>(new Date(2025, 5, 12));

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    
    const finalUserData = {
      ...userData,
      birthday: date || new Date()
    };
    
    console.log("Form data:", finalUserData);
    setUserData(finalUserData);
  };

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0] || null;
    setUserData(prev => ({ ...prev, profilePicture: file }));
  };

  return (
    <div className="flex flex-col gap-4 items-center h-screen w-full p-8">
      <h1 className="text-4xl font-bold">Welcome to Y</h1>
      <p className="text-2xl font-medium">Share your world, discover new voices, and build your digital presence with purpose.</p>
      <div className="w-96 bg-stone-100 rounded-2xl p-4">
        <Label htmlFor="file-input" className="flex flex-col items-center cursor-pointer p-4">
          <Avatar className="w-24 h-24 rounded-full mb-4 bg-gray-300" />
          <span>Upload a profile picture</span>
        </Label>
        <Input 
          type="file" 
          accept="image/*" 
          name="profilePicture" 
          id="file-input" 
          className="mb- hidden" 
          onChange={handleFileChange}
        />
        <div className="flex  items-center gap-2">
          <p className="mb-4 text-blue-700">@y/</p>
           <Input 
          type="text" 
          placeholder="Full Name" 
          className="mb-4" 
          name="name" 
          value={userData.name} 
          onChange={(e) => setUserData({ ...userData, name: e.target.value })} 
        />
        </div>
       
        <Calendar 
          mode="single" 
          defaultMonth={date} 
          selected={date} 
          onSelect={setDate} 
          captionLayout={dropdown} 
          className="mb-4 mx-auto w-full rounded-md" 
        />
        <div className="flex flex-col gap-3 hidden">
          <Label htmlFor="dropdown" className="px-1">
            Date Selection:
          </Label>
          <Select 
            value={dropdown} 
            onValueChange={(value) => setDropdown(value as React.ComponentProps<typeof Calendar>["captionLayout"])}
          >
            <SelectTrigger id="dropdown" size="sm" className="bg-background w-full">
              <SelectValue placeholder="Dropdown" />
            </SelectTrigger>
            <SelectContent align="center">
              <SelectItem value="dropdown">Month and Year</SelectItem>
              <SelectItem value="dropdown-months">Month Only</SelectItem>
              <SelectItem value="dropdown-years">Year Only</SelectItem>
            </SelectContent>
          </Select>
        </div>
        <Textarea 
          placeholder="Bio" 
          className="mb-4" 
          name="bio" 
          value={userData.bio} 
          onChange={(e) => setUserData({ ...userData, bio: e.target.value })} 
        />
        <Button 
          type="submit" 
          className="flex items-center justify-center gap-2 mx-auto w-full" 
          onClick={handleSubmit}
        >
          Next
          <ChevronRight
            size={16}
            strokeWidth={2.5}
            style={{ marginLeft: 2 }}         
          />                    
        </Button>
      </div>
    </div>
  );
};

export default OnBoardingPage;