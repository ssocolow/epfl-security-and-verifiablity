"use client"

import { createContext, useContext, useState, type ReactNode } from "react"

export interface Question {
  id: number
  text: string
  selected?: boolean
  answer?: boolean
}

interface MeetContextType {
  meetCode: string
  description: string
  questions: Question[]
  setMeetCode: (code: string) => void
  setDescription: (description: string) => void
  toggleQuestionSelection: (id: number) => void
  setQuestionAnswer: (id: number, answer: boolean) => void
  getSelectedQuestions: () => Question[]
  resetMeet: () => void
}

const defaultQuestions: Question[] = [
  { id: 1, text: "I have gone hiking in the past year." },
  { id: 2, text: "I own at least one physical book." },
  { id: 3, text: "I have played a video game in the last month." },
  { id: 4, text: "I have had a deep conversation about life or philosophy recently." },
  { id: 5, text: "I have tried a cuisine from another country in the past three months." },
  { id: 6, text: "I can speak more than one language fluently." },
  { id: 7, text: "I have owned a pet at some point in my life." },
  { id: 8, text: "I have listened to a podcast or audiobook in the past month." },
  { id: 9, text: "I have attended a live concert, festival, or theater performance in the past year." },
  { id: 10, text: "I have exercised at least three times in the past week." },
  { id: 11, text: "I have read or watched news about world events this week." },
  { id: 12, text: "I have completed a puzzle, riddle, or brain teaser in the last month." },
  { id: 13, text: "I have woken up before 6 AM at least once this past week." },
  { id: 14, text: "I have introduced myself to a new person in the last month." },
  { id: 15, text: "I have used or learned about AI, blockchain, or VR in the past year." },
]

const MeetContext = createContext<MeetContextType | undefined>(undefined)

export function MeetProvider({ children }: { children: ReactNode }) {
  const [meetCode, setMeetCode] = useState(() => {
    // Generate a random 6-character code
    return Math.random().toString(36).substring(2, 8).toUpperCase()
  })
  const [description, setDescription] = useState("")
  const [questions, setQuestions] = useState<Question[]>(defaultQuestions)

  const toggleQuestionSelection = (id: number) => {
    setQuestions(questions.map((q) => (q.id === id ? { ...q, selected: !q.selected } : q)))
  }

  const setQuestionAnswer = (id: number, answer: boolean) => {
    setQuestions(questions.map((q) => (q.id === id ? { ...q, answer } : q)))
  }

  const getSelectedQuestions = () => {
    return questions.filter((q) => q.selected)
  }

  const resetMeet = () => {
    setMeetCode(Math.random().toString(36).substring(2, 8).toUpperCase())
    setDescription("")
    setQuestions(defaultQuestions)
  }

  return (
    <MeetContext.Provider
      value={{
        meetCode,
        description,
        questions,
        setMeetCode,
        setDescription,
        toggleQuestionSelection,
        setQuestionAnswer,
        getSelectedQuestions,
        resetMeet,
      }}
    >
      {children}
    </MeetContext.Provider>
  )
}

export function useMeet() {
  const context = useContext(MeetContext)
  if (context === undefined) {
    throw new Error("useMeet must be used within a MeetProvider")
  }
  return context
}

