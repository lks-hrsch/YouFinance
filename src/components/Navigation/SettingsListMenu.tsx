import { ChevronDownIcon } from "@heroicons/react/24/outline";

import {
  Collapse,
  ListItem,
  Menu,
  MenuHandler,
  MenuItem,
  MenuList,
  Typography,
} from "@material-tailwind/react";
import React from "react";

const navListMenuItems = [
  {
    title: "BankAcountDataProvider",
    description: "Configure APIs to retrieve bank account data.",
    href: "/settings/bank-account-data-provider",
  },
  {
    title: "BankAccounts",
    description: "Configure Bank accounts",
    href: "/settings/bank-accounts",
  },
];

const SettingsListMenu: React.FC = () => {
  const [isMenuOpen, setIsMenuOpen] = React.useState(false);
  const [isMobileMenuOpen, setIsMobileMenuOpen] = React.useState(false);

  const renderItems = navListMenuItems.map(
    ({ title, description, href }, key) => (
      <a href={href} key={key}>
        <MenuItem
          className="flex items-center gap-3 rounded-lg"
          onPointerEnterCapture={undefined}
          onPointerLeaveCapture={undefined}
          placeholder={undefined}
        >
          <div>
            <Typography
              className="flex items-center font-bold text-sm"
              color="blue-gray"
              onPointerEnterCapture={undefined}
              onPointerLeaveCapture={undefined}
              placeholder={undefined}
              variant="h6"
            >
              {title}
            </Typography>
            <Typography
              className="!font-medium text-blue-gray-500 text-xs"
              onPointerEnterCapture={undefined}
              onPointerLeaveCapture={undefined}
              placeholder={undefined}
              variant="paragraph"
            >
              {description}
            </Typography>
          </div>
        </MenuItem>
      </a>
    )
  );

  return (
    <React.Fragment>
      <Menu
        handler={setIsMenuOpen}
        offset={{ mainAxis: 20 }}
        open={isMenuOpen}
        placement="bottom"
      >
        <MenuHandler>
          <Typography
            as="div"
            className="font-medium"
            onPointerEnterCapture={undefined}
            onPointerLeaveCapture={undefined}
            placeholder={undefined}
            variant="small"
          >
            <ListItem
              className="flex items-center gap-2 py-2 pr-4 font-medium text-gray-900"
              onClick={() => setIsMobileMenuOpen((cur) => !cur)}
              onPointerEnterCapture={undefined}
              onPointerLeaveCapture={undefined}
              placeholder={undefined}
              selected={isMenuOpen || isMobileMenuOpen}
            >
              Settings
              <ChevronDownIcon
                className={`hidden h-3 w-3 transition-transform lg:block ${
                  isMenuOpen ? "rotate-180" : ""
                }`}
                strokeWidth={2.5}
              />
              <ChevronDownIcon
                className={`block h-3 w-3 transition-transform lg:hidden ${
                  isMobileMenuOpen ? "rotate-180" : ""
                }`}
                strokeWidth={2.5}
              />
            </ListItem>
          </Typography>
        </MenuHandler>
        <MenuList
          className="hidden max-w-screen-xl rounded-xl lg:block"
          onPointerEnterCapture={undefined}
          onPointerLeaveCapture={undefined}
          placeholder={undefined}
        >
          <ul className="grid grid-cols-3 gap-y-2 outline-none outline-0">
            {renderItems}
          </ul>
        </MenuList>
      </Menu>
      <div className="block lg:hidden">
        <Collapse open={isMobileMenuOpen}>{renderItems}</Collapse>
      </div>
    </React.Fragment>
  );
};

export default SettingsListMenu;
