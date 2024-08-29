/*
  Warnings:

  - You are about to drop the column `account_name` on the `accounts` table. All the data in the column will be lost.
  - Added the required column `username` to the `accounts` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "accounts" DROP COLUMN "account_name",
ADD COLUMN     "username" TEXT NOT NULL;
