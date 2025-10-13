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
import useAPI from "@/api/use-api";
import ROUTE from "@/api/route";

const OnBoardingPage = () => {
  const [userData, setUserData] = useState<{ name: string; bio: string; birthday: Date; profilePicture: File | null }>({
    name: "",
    bio: "",
    birthday: new Date(),
    profilePicture: null,
  });
  const [dropdown, setDropdown] = useState<React.ComponentProps<typeof Calendar>["captionLayout"]>("dropdown");
  const [date, setDate] = useState<Date | undefined>(new Date());

  const { mutateAsync, mutation } = useAPI<undefined, FormData, { ok?: boolean }>(ROUTE.USER_ONBOARDING, {
    as: "formData",
  });

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    const fd = new FormData();
    // Backend expects: username, bio, dob, profile
    fd.append("username", userData.name.trim());
    fd.append("bio", userData.bio.trim());
    if (date) {
      // Send as YYYY-MM-DD (backend parses NaiveDate or RFC3339)
      const yyyyMmDd = new Date(date).toISOString().split("T")[0];
      fd.append("dob", yyyyMmDd);
    }
    if (userData.profilePicture) {
      fd.append("profile", userData.profilePicture);
    }

    try {
      await mutateAsync(fd);
      // TODO: navigate to next step/page if needed
      // router.replace("/next");
    } catch (err) {
      // Error is already surfaced by mutation.isError
      console.error(err);
    }
  };

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0] || null;
    setUserData((prev) => ({ ...prev, profilePicture: file }));
  };

  return (
    <div className="flex flex-col gap-4 items-center h-screen w-full p-8">
      <h1 className="text-4xl font-bold">Welcome to Y</h1>
      <p className="text-2xl font-medium">Share your world, discover new voices, and build your digital presence with purpose.</p>
      <form className="w-96 bg-stone-100 rounded-2xl p-4" onSubmit={handleSubmit}>
        <Label htmlFor="file-input" className="flex flex-col items-center cursor-pointer p-4">
          <Avatar className="w-24 h-24 rounded-full mb-4 bg-gray-300" />
          <span>Upload a profile picture</span>
        </Label>
        <Input type="file" accept="image/*" name="profilePicture" id="file-input" className="mb- hidden" onChange={handleFileChange} />
        <div className="flex  items-center gap-2">
          <p className="mb-4 text-blue-700">@y/</p>
          <Input
            type="text"
            placeholder="Username"
            className="mb-4"
            name="name"
            value={userData.name}
            onChange={(e) => setUserData({ ...userData, name: e.target.value })}
          />
        </div>

        <Calendar mode="single" defaultMonth={date} selected={date} onSelect={setDate} captionLayout={dropdown} className="mb-4 mx-auto w-full rounded-md" />
        <div className="flex flex-col gap-3 hidden">
          <Label htmlFor="dropdown" className="px-1">
            Date Selection:
          </Label>
          <Select value={dropdown} onValueChange={(value) => setDropdown(value as React.ComponentProps<typeof Calendar>["captionLayout"]) }>
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
        <Textarea placeholder="Bio" className="mb-4" name="bio" value={userData.bio} onChange={(e) => setUserData({ ...userData, bio: e.target.value })} />
        <Button type="submit" className="flex items-center justify-center gap-2 mx-auto w-full" disabled={mutation.isPending}>
          {mutation.isPending ? "Saving…" : "Next"}
          <ChevronRight size={16} strokeWidth={2.5} style={{ marginLeft: 2 }} />
        </Button>
        {mutation.isError && <p className="text-red-600 mt-2">Failed to save onboarding</p>}
      </form>
    </div>
  );
};

export default OnBoardingPage;