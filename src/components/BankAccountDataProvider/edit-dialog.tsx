"use client";

import { invoke } from "@tauri-apps/api/core";
import { Eye, EyeOff } from "lucide-react";
import type React from "react";
import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import type { Provider } from "@/models/typeshare_definitions";

interface EditDialogProps {
  onClose: () => void;
  onSaved: () => void;
  provider: Provider | null;
}

const ProviderEditDialog: React.FC<EditDialogProps> = ({
  provider,
  onClose,
  onSaved,
}) => {
  const [sid, setSid] = useState<string>("");
  const [skey, setSkey] = useState<string>("");
  const [showPassword, setShowPassword] = useState<boolean>(false);
  const [error, setError] = useState<string>("");
  const [isSaving, setIsSaving] = useState<boolean>(false);

  useEffect(() => {
    if (provider) {
      setError("");
      setSid("");
      setSkey("");
      setShowPassword(false);

      // Parse config_json to extract credentials
      try {
        const config = JSON.parse(provider.config_json);
        setSid(config.secret_id || "");
        setSkey(config.secret_key || "");
      } catch {
        // If config is invalid, just keep empty
      }
    }
  }, [provider]);

  const handleSave = async () => {
    if (!provider) {
      return;
    }

    setError("");
    setIsSaving(true);

    try {
      await invoke("update_banking_provider", {
        providerId: provider.id,
        sid,
        skey,
      });
      onSaved();
      onClose();
    } catch (err) {
      setError(
        err instanceof Error ? err.message : "Failed to update provider"
      );
    } finally {
      setIsSaving(false);
    }
  };

  const isOpen = provider !== null;
  const isGoCardless = provider?.name === "GoCardless";

  return (
    <Dialog onOpenChange={() => !isSaving && onClose()} open={isOpen}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Edit {provider?.name} Provider</DialogTitle>
          {isGoCardless ? (
            <DialogDescription>
              Update your GoCardless API credentials
            </DialogDescription>
          ) : (
            <DialogDescription>
              This provider has no configurable credentials.
            </DialogDescription>
          )}
        </DialogHeader>

        {isGoCardless ? (
          <div className="space-y-4">
            <div className="space-y-2">
              <Label htmlFor="secret-id">Secret ID</Label>
              <Input
                id="secret-id"
                onChange={(e) => setSid(e.target.value)}
                placeholder="Enter your GoCardless Secret ID"
                value={sid}
              />
            </div>

            <div className="space-y-2">
              <Label htmlFor="secret-key">Secret Key</Label>
              <div className="flex gap-2">
                <Input
                  id="secret-key"
                  onChange={(e) => setSkey(e.target.value)}
                  placeholder="Enter your GoCardless Secret Key"
                  type={showPassword ? "text" : "password"}
                  value={skey}
                />
                <Button
                  onClick={() => setShowPassword(!showPassword)}
                  size="icon"
                  title={showPassword ? "Hide password" : "Show password"}
                  type="button"
                  variant="outline"
                >
                  {showPassword ? (
                    <EyeOff className="size-4" />
                  ) : (
                    <Eye className="size-4" />
                  )}
                </Button>
              </div>
            </div>

            {error && (
              <div className="rounded border border-red-200 bg-red-50 p-3 text-red-700 text-sm">
                {error}
              </div>
            )}

            <div className="flex justify-end gap-2 pt-2">
              <Button
                disabled={isSaving}
                onClick={onClose}
                type="button"
                variant="outline"
              >
                Cancel
              </Button>
              <Button disabled={isSaving} onClick={handleSave}>
                {isSaving ? "Saving..." : "Save"}
              </Button>
            </div>
          </div>
        ) : (
          <div className="flex justify-end gap-2 pt-2">
            <Button onClick={onClose} variant="outline">
              Close
            </Button>
          </div>
        )}
      </DialogContent>
    </Dialog>
  );
};

export default ProviderEditDialog;
