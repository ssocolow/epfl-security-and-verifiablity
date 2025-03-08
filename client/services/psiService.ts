// services/psiService.ts
export interface PsiRequestData {
    public_key: string;
    encrypted_interests: string[];
    merkle_root: string;
  }
  
  export interface PsiResponse {
    status: string;
    message: string;
    fhe_result: string[];
  }
  
  export async function processPSI(data: PsiRequestData): Promise<PsiResponse> {
    const response = await fetch("http://127.0.0.1:8080/process-psi", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    });
  
    if (!response.ok) {
      throw new Error("Failed to process PSI");
    }
    return response.json();
  }