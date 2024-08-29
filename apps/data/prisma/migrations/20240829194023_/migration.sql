-- CreateEnum
CREATE TYPE "HashingAlgorithm" AS ENUM ('SHA1', 'SHA256', 'SHA512');

-- CreateTable
CREATE TABLE "users" (
    "id" TEXT NOT NULL,

    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "accounts" (
    "id" TEXT NOT NULL,
    "digits" SMALLINT NOT NULL,
    "skew" SMALLINT NOT NULL,
    "step" SMALLINT NOT NULL,
    "secret" VARCHAR(256) NOT NULL,
    "issuer" TEXT NOT NULL,
    "account_name" TEXT NOT NULL,
    "algorithm" "HashingAlgorithm" NOT NULL,

    CONSTRAINT "accounts_pkey" PRIMARY KEY ("id")
);
