import { Loader2, RefreshCw } from "lucide-react";
import { useState } from "react";
import { toast } from "sonner";
import { Button } from "@/common/component/ui/button";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@/common/component/ui/card";
import { allMdwtTagRefresh } from "../service";

export default function FixDb() {
  const [isLoading, setIsLoading] = useState(false);

  const handleRefreshTags = async () => {
    setIsLoading(true);
    try {
      await allMdwtTagRefresh();
      toast.success("Tags refreshed successfully");
    } catch (error) {
      toast.error("Failed to refresh tags");
      console.error(error);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="container mx-auto py-8">
      <Card className="max-w-2xl mx-auto">
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <RefreshCw className="h-5 w-5" />
            Fix Database
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <p className="text-sm text-muted-foreground">
            Refresh all markdown tags across the system. This will recalculate
            and update tag associations for all markdown notes.
          </p>
          <Button
            onClick={handleRefreshTags}
            disabled={isLoading}
            className="w-full"
          >
            {isLoading ? (
              <>
                <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                Refreshing Tags...
              </>
            ) : (
              <>
                <RefreshCw className="mr-2 h-4 w-4" />
                Refresh All Tags
              </>
            )}
          </Button>
        </CardContent>
      </Card>
    </div>
  );
}
