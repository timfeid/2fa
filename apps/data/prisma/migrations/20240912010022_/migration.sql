/*
  Warnings:

  - Changed the type of `algorithm` on the `accounts` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.

*/
-- CreateEnum
CREATE TYPE "hashing_algorithm" AS ENUM ('SHA1', 'SHA256', 'SHA512');

-- AlterTable
ALTER TABLE "accounts" DROP COLUMN "algorithm",
ADD COLUMN     "algorithm" "hashing_algorithm" NOT NULL;

-- DropEnum
DROP TYPE "HashingAlgorithm";
