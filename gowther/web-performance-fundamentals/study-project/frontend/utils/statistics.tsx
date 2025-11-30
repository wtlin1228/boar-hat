import { Button } from "@fe/components/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@fe/components/dropdown-menu";
import { PillLightCoral, PillLightGreen } from "@fe/components/pill/colorful";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeadCell,
  TableRow,
} from "@fe/components/table";
import { TableData } from "@fe/data/website-statistics";
import { DotsVerticalIcon } from "@fe/icons/dots-vertical-icon";

export const renderTable = (tableData: TableData[]) => {
  return (
    <Table className="min-w-[44rem]">
      <TableHead>
        <TableRow>
          <TableHeadCell>Source</TableHeadCell>
          <TableHeadCell>Visitors</TableHeadCell>
          <TableHeadCell>Revenue</TableHeadCell>
          <TableHeadCell>Status</TableHeadCell>
          <TableHeadCell>Action</TableHeadCell>
        </TableRow>
      </TableHead>
      <TableBody>
        {tableData.map((row, index) => (
          <TableRow key={index}>
            <TableCell>{row.source}</TableCell>
            <TableCell>{row.visitors}</TableCell>
            <TableCell>{row.revenue}</TableCell>
            <TableCell>
              {row.status === "up" ? (
                <PillLightGreen>{row.statusText}</PillLightGreen>
              ) : (
                <PillLightCoral>{row.statusText}</PillLightCoral>
              )}
            </TableCell>
            <TableCell>
              <DropdownMenu
                trigger={
                  <DropdownMenuTrigger>
                    <Button appearance="text" className="w-10">
                      <DotsVerticalIcon className="w-8 h-8 shrink-0" />
                    </Button>
                  </DropdownMenuTrigger>
                }
              >
                <DropdownMenuContent>
                  <DropdownMenuItem>View details</DropdownMenuItem>
                  <DropdownMenuItem>Add to watchlist</DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            </TableCell>
          </TableRow>
        ))}
      </TableBody>
    </Table>
  );
};

export const TableSkeleton = () => {
  return (
    <div className="animate-pulse">
      {/* Header row */}
      <div className="flex border-b border-blinkGray100 dark:border-blinkNeutral700">
        {[...Array(5)].map((_, i) => (
          <div key={i} className="flex-1 p-3">
            <div className="h-5 bg-blinkGray200 dark:bg-blinkNeutral700 rounded w-20"></div>
          </div>
        ))}
      </div>

      {/* Table rows */}
      {[...Array(8)].map((_, rowIndex) => (
        <div
          key={rowIndex}
          className="flex border-b border-blinkGray100 dark:border-blinkNeutral700"
        >
          {/* Source cell */}
          <div className="flex-1 p-3">
            <div className="h-5 bg-blinkGray200 dark:bg-blinkNeutral700 rounded w-24"></div>
          </div>

          {/* Visitors cell */}
          <div className="flex-1 p-3">
            <div className="h-5 bg-blinkGray200 dark:bg-blinkNeutral700 rounded w-16"></div>
          </div>

          {/* Revenue cell */}
          <div className="flex-1 p-3">
            <div className="h-5 bg-blinkGray200 dark:bg-blinkNeutral700 rounded w-14"></div>
          </div>

          {/* Status cell */}
          <div className="flex-1 p-3">
            <div className="h-6 bg-blinkGray200 dark:bg-blinkNeutral700 rounded w-12"></div>
          </div>

          {/* Action cell */}
          <div className="flex-1 p-3 flex justify-center">
            <div className="h-8 w-8 bg-blinkGray200 dark:bg-blinkNeutral700 rounded-full"></div>
          </div>
        </div>
      ))}
    </div>
  );
};
