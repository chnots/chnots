import KSpaceSettings from "@/krate/kspace/component/settings";
import { useState } from "react";
import KListItem from "../component/klistitem";
import { useCommonStore } from "@/common/store";

type SettingsType = "kspace" | "profile";

interface SettingsNavItem {
  label: SettingsType;
}

const SettingsPage = () => {
  const [activeTab, setActiveTab] = useState<SettingsType>("profile");
  const { showSidebar } = useCommonStore();
  const navItems: SettingsNavItem[] = [
    {
      label: "kspace",
    },
  ];

  const handleTabClick = (tabId: SettingsType) => {
    setActiveTab(tabId);
  };

  const renderSettingsContent = () => {
    switch (activeTab) {
      case "kspace":
        return <KSpaceSettings />;
      default:
        return <ProfileSettings />;
    }
  };

  return (
    <div className="flex h-screen bg-gray-50">
      {/* Sidebar */}
      {showSidebar && (
        <div className="w-64 bg-white border-r border-gray-200 p-4 overflow-y-auto">
          <h2 className="text-xl font-bold mb-6 text-gray-800">Settings</h2>
          <nav>
            <ul className="space-y-2">
              {navItems.map((item) => (
                <KListItem
                  key={item.label}
                  onClick={() => handleTabClick(item.label)}
                >
                  <div>{item.label}</div>
                </KListItem>
              ))}
            </ul>
          </nav>
        </div>
      )}

      {/* Main Content */}
      <div className="flex-1 p-8 overflow-y-auto">
        <div className="max-w-3xl mx-auto bg-white rounded-lg shadow-sm p-6">
          {renderSettingsContent()}
        </div>
      </div>
    </div>
  );
};

// Example settings components
const ProfileSettings = () => (
  <div>
    <h3 className="text-lg font-semibold mb-4">Profile Settings</h3>
    <div className="space-y-4">
      <div>
        <label className="block text-sm font-medium text-gray-700">Name</label>
        <input
          type="text"
          className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
          defaultValue="John Doe"
        />
      </div>
      <div>
        <label className="block text-sm font-medium text-gray-700">Bio</label>
        <textarea
          rows={3}
          className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
          defaultValue="Software developer and designer"
        />
      </div>
    </div>
  </div>
);

export default SettingsPage;
